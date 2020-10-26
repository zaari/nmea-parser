/*
Copyright 2020 Timo Saarinen

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
use super::*;

use std::num::ParseIntError;

const AIS_CHAR_BITS: usize = 6;

/// Make a key for storing NMEA sentence fragments
pub(crate) fn make_fragment_key(
    sentence_type: &str,
    message_id: u64,
    fragment_count: u8,
    fragment_number: u8,
    radio_channel_code: &str,
) -> String {
    format!(
        "{},{},{},{},{}",
        sentence_type, fragment_count, fragment_number, message_id, radio_channel_code
    )
}

/// Converts AIVDM playload armored string into bit vector.
pub(crate) fn parse_payload(payload: &String) -> Result<BitVec, String> {
    let mut bv = BitVec::<LocalBits, usize>::with_capacity(payload.len() * 6); // or Lsb0 or Msb0 ?
    for c in payload.chars() {
        let mut ci = (c as u8) - 48;
        if ci > 40 {
            ci -= 8;
        }

        // Pick bits
        for i in 0..6 {
            bv.push(((ci >> (5 - i)) & 0x01) != 0);
        }
    }

    Ok(bv)
}

/// Picks a numberic field from BitVec.
pub(crate) fn pick_u64(bv: &BitVec, index: usize, len: usize) -> u64 {
    let mut res = 0;
    for pos in index..(index + len) {
        res = (res << 1) | (*bv.get(pos).unwrap_or(&false) as u64);
    }
    res
}

/// Picks a signed numberic field from BitVec.
pub(crate) fn pick_i64(bv: &BitVec, index: usize, len: usize) -> i64 {
    let mut res = 0;
    for pos in index..(index + len) {
        res = (res << 1) | (*bv.get(pos).unwrap_or(&false) as u64);
    }

    let sign_bit = 1 << (len - 1);
    if res & sign_bit != 0 {
        ((res & (sign_bit - 1)) as i64) - (sign_bit as i64)
    } else {
        res as i64
    }
}

/// Pick a string from BitVec. Char_count is the length in characters. Character size is 6-bits.
pub(crate) fn pick_string(bv: &BitVec, index: usize, char_count: usize) -> String {
    let mut res = String::with_capacity(char_count);
    for i in 0..char_count {
        // unwraps below won't panic as char_from::u32 will only ever receive values between
        // 32..=96, all of which are valid. Catch all branch is unreachable as we only request
        // 6-bits from the BitVec.
        match pick_u64(bv, index + i * AIS_CHAR_BITS, AIS_CHAR_BITS) as u32 {
            0 => break,
            ch if ch < 32 => res.push(std::char::from_u32(64 + ch).unwrap()),
            ch if ch < 64 => res.push(std::char::from_u32(ch).unwrap()),
            ch => unreachable!("6-bit AIS character expected but value {} encountered!", ch),
        }
    }

    let trimmed_len = res.trim_end().len();
    res.truncate(trimmed_len);
    res
}

/// Pick ETA based on UTC month, day, hour and minute.
pub(crate) fn pick_eta(bv: &BitVec, index: usize) -> Option<DateTime<Utc>> {
    let now = Utc::now().naive_utc();

    // Pick ETA
    let mut month = pick_u64(bv, index, 4) as u32;
    let mut day = pick_u64(bv, index + 4, 5) as u32;
    let mut hour = pick_u64(bv, index + 4 + 5, 5) as u32;
    let mut minute = pick_u64(bv, index + 4 + 5 + 5, 6) as u32;

    if month == 0 && day == 0 && hour == 24 && minute == 60 {
        return None;
    }

    if month == 0 {
        month = now.month();
    }
    if day == 0 {
        day = now.day();
    }
    if hour == 24 {
        hour = 23;
        minute = 59;
    }
    if minute == 60 {
        minute = 59;
    }

    // This and next year
    let this_year_eta = NaiveDate::from_ymd(now.year(), month, day).and_hms(hour, minute, 30);
    let next_year_eta = NaiveDate::from_ymd(now.year(), month, day).and_hms(hour, minute, 30);

    if now <= this_year_eta {
        Some(DateTime::<Utc>::from_utc(this_year_eta, Utc))
    } else {
        Some(DateTime::<Utc>::from_utc(next_year_eta, Utc))
    }
}

/// Pick field from comma-separated sentence or None if empty field.
pub(crate) fn pick_number_field<T: std::str::FromStr>(
    split: &[&str],
    num: usize,
) -> Result<Option<T>, String> {
    split
        .get(num)
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse()
                .map_err(|_| format!("Failed to parse field {}: {}", num, s))
        })
        .transpose()
}

/// Parse time field of format HHMMSS and convert it to DateTime<Utc> using the current time.
pub(crate) fn parse_hhmmss(hhmmss: &str, now: DateTime<Utc>) -> Result<DateTime<Utc>, String> {
    let (hour, minute, second) =
        parse_time(hhmmss).map_err(|_| format!("Invalid time format: {}", hhmmss))?;
    Ok(Utc
        .ymd(now.year(), now.month(), now.day())
        .and_hms(hour, minute, second))
}

/// Parse time fields of formats YYMMDD and HHMMSS and convert them to DateTime<Utc>.
pub(crate) fn parse_yymmdd_hhmmss(yymmdd: &str, hhmmss: &str) -> Result<DateTime<Utc>, String> {
    let century = (Utc::now().year() / 100) * 100;
    let (day, month, year) =
        parse_date(yymmdd).map_err(|_| format!("Invalid date format: {}", yymmdd))?;
    let (hour, minute, second) =
        parse_time(hhmmss).map_err(|_| format!("Invalid time format: {}", hhmmss))?;
    Ok(Utc
        .ymd(century + year, month, day)
        .and_hms(hour, minute, second))
}

/// Parse day, month and year from YYMMDD string.
fn parse_date(yymmdd: &str) -> Result<(u32, u32, i32), ParseIntError> {
    let day = pick_s2(yymmdd, 0).parse::<u32>()?;
    let month = pick_s2(yymmdd, 2).parse::<u32>()?;
    let year = pick_s2(yymmdd, 4).parse::<i32>()?;
    Ok((day, month, year))
}

/// Parse hour, minute and second from HHMMSS string.
fn parse_time(hhmmss: &str) -> Result<(u32, u32, u32), ParseIntError> {
    let hour = pick_s2(hhmmss, 0).parse::<u32>()?;
    let minute = pick_s2(hhmmss, 2).parse::<u32>()?;
    let second = pick_s2(hhmmss, 4).parse::<u32>()?;
    Ok((hour, minute, second))
}

/// A simple helper to pick a substring of length two from the given string.
fn pick_s2(s: &str, i: usize) -> &str {
    let end = i + 2;
    s.get(i..end).unwrap_or("")
}

/// Parse latitude from two string.
/// lat_string = DDMM.MMM representing latitude
/// hemisphere = N for north, S for south
pub(crate) fn parse_latitude_ddmm_mmm(
    lat_string: &str,
    hemisphere: &str,
) -> Result<Option<f64>, String> {
    // DDMM.MMM
    if lat_string.is_empty() {
        return Ok(None);
    }

    // Validate: 4 digits, a decimal point, then 1 or more digits
    let byte_string = lat_string.as_bytes();
    if !(byte_string.iter().take(4).all(|c| c.is_ascii_digit())
        && byte_string.get(4) == Some(&b'.')
        && byte_string
            .get(5)
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false))
    {
        return Err(format!(
            "Failed to parse latitude (DDMM.MMM) from {}",
            lat_string
        ));
    }
    let end = 5 + byte_string
        .iter()
        .skip(5)
        .take_while(|c| c.is_ascii_digit())
        .count();

    // Extract
    let d = lat_string[0..2].parse::<f64>().unwrap_or(0.0);
    let m = lat_string[2..end].parse::<f64>().unwrap_or(0.0);
    let val = d + m / 60.0;
    Ok(Some(match hemisphere {
        "N" => val,
        "S" => -val,
        _ => val,
    }))
}

/// Parse longitude from two string.
/// lon_string = DDDMM.MMM representing latitude
/// eastwest = E for north, W for south
pub(crate) fn parse_longitude_dddmm_mmm(
    lon_string: &str,
    eastwest: &str,
) -> Result<Option<f64>, String> {
    // DDDMM.MMM
    if lon_string.is_empty() {
        return Ok(None);
    }

    // Validate: 5 digits, a decimal point, then 1 or more digits
    let byte_string = lon_string.as_bytes();
    if !(byte_string.iter().take(5).all(|c| c.is_ascii_digit())
        && byte_string.get(5) == Some(&b'.')
        && byte_string
            .get(6)
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false))
    {
        return Err(format!(
            "Failed to parse longitude (DDDMM.MMM) from {}",
            lon_string
        ));
    }
    let end = 6 + byte_string
        .iter()
        .skip(6)
        .take_while(|c| c.is_ascii_digit())
        .count();

    // Extract
    let d = lon_string[0..3].parse::<f64>().unwrap_or(0.0);
    let m = lon_string[3..end].parse::<f64>().unwrap_or(0.0);
    let val = d + m / 60.0;
    Ok(Some(match eastwest {
        "E" => val,
        "W" => -val,
        _ => val,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_payload() {
        match parse_payload(&"w7b0P1".to_string()) {
            Ok(bv) => {
                assert_eq!(
                    bv,
                    bits![
                        1, 1, 1, 1, 1, 1, //
                        0, 0, 0, 1, 1, 1, //
                        1, 0, 1, 0, 1, 0, //
                        0, 0, 0, 0, 0, 0, //
                        1, 0, 0, 0, 0, 0, //
                        0, 0, 0, 0, 0, 1, //
                    ]
                );
            }
            Err(e) => {
                assert_eq!(e, "OK");
            }
        }
    }

    #[test]
    fn test_pick_u64() {
        let bv = bitvec![1, 0, 1, 1, 0, 1];
        assert_eq!(pick_u64(&bv, 0, 2), 2);
        assert_eq!(pick_u64(&bv, 2, 2), 3);
        assert_eq!(pick_u64(&bv, 4, 2), 1);
        assert_eq!(pick_u64(&bv, 0, 6), 45);
        assert_eq!(pick_u64(&bv, 4, 4), 4);
        assert_eq!(pick_u64(&bv, 6, 2), 0);
    }

    #[test]
    fn test_pick_i64() {
        assert_eq!(pick_i64(&bitvec![0, 1, 1, 1, 1, 1], 0, 6), 31);
        assert_eq!(pick_i64(&bitvec![0, 0, 0, 0, 0, 1], 0, 6), 1);
        assert_eq!(pick_i64(&bitvec![0, 0, 0, 0, 0, 0], 0, 6), 0);
        assert_eq!(pick_i64(&bitvec![1, 1, 1, 1, 1, 1], 0, 6), -1);
        assert_eq!(pick_i64(&bitvec![1, 0, 0, 0, 0, 0], 0, 6), -32);
    }

    #[test]
    fn test_pick_string() {
        let bv = bitvec![
            1, 1, 1, 1, 1, 1, // ?
            0, 0, 0, 0, 0, 1, // A
            0, 0, 0, 1, 1, 1, // G
            0, 1, 1, 1, 1, 1, // _
            1, 1, 0, 1, 0, 0, // 4
            1, 1, 1, 0, 1, 0, // :
            1, 0, 0, 0, 0, 1, // !
            0, 0, 0, 0, 0, 0, // @ (end of line char)
            0, 0, 0, 0, 1, 0, // B (rubbish)
        ];
        assert_eq!(pick_string(&bv, 0, bv.len() / 6), "?AG_4:!");
    }

    #[test]
    fn test_pick_number_field() {
        let s: Vec<&str> = "128,0,8.0,xyz".split(',').collect();
        assert_eq!(pick_number_field::<u8>(&s, 0).ok().unwrap().unwrap(), 128);
        assert_eq!(pick_number_field::<u8>(&s, 1).ok().unwrap().unwrap(), 0);
        assert_eq!(pick_number_field::<f64>(&s, 2).ok().unwrap().unwrap(), 8.0);
        assert_eq!(pick_number_field::<f64>(&s, 3).is_ok(), false);
        assert_eq!(pick_number_field::<f64>(&s, 4).ok().unwrap(), None);
    }
}
