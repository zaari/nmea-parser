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

use chrono::Duration;

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

/// Convert AIS VDM/VDO payload armored string into a `BitVec`.
pub(crate) fn parse_payload(payload: &str) -> Result<BitVec, String> {
    let mut bv = BitVec::<LocalBits, usize>::with_capacity(payload.len() * 6);
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

/// Pick a numberic field from `BitVec`.
pub(crate) fn pick_u64(bv: &BitVec, index: usize, len: usize) -> u64 {
    let mut res = 0;
    for pos in index..(index + len) {
        if let Some(b) = bv.get(pos) {
            res = (res << 1) | (*b as u64);
        } else {
            res <<= 1;
        }
    }
    res
}

/// Pick a signed numberic field from `BitVec`.
pub(crate) fn pick_i64(bv: &BitVec, index: usize, len: usize) -> i64 {
    let mut res = 0;
    for pos in index..(index + len) {
        if let Some(b) = bv.get(pos) {
            res = (res << 1) | (*b as u64);
        } else {
            res <<= 1;
        }
    }

    let sign_bit = 1 << (len - 1);
    if res & sign_bit != 0 {
        ((res & (sign_bit - 1)) as i64) - (sign_bit as i64)
    } else {
        res as i64
    }
}

/// Pick a string from BitVec. Field `char_count` defines string length in characters.
/// Characters consist of 6 bits.
pub(crate) fn pick_string(bv: &BitVec, index: usize, char_count: usize) -> String {
    let mut res = String::with_capacity(char_count);
    for i in 0..char_count {
        // unwraps below won't panic as char_from::u32 will only ever receive values between
        // 32..=96, all of which are valid. Catch all branch is unreachable as we only request
        // 6-bits from the BitVec.
        match pick_u64(bv, index + i * AIS_CHAR_BITS, AIS_CHAR_BITS) as u32 {
            0 => break,
            ch if ch < 32 => res.push(core::char::from_u32(64 + ch).unwrap()),
            ch if ch < 64 => res.push(core::char::from_u32(ch).unwrap()),
            ch => unreachable!("6-bit AIS character expected but value {} encountered!", ch),
        }
    }

    let trimmed_len = res.trim_end().len();
    res.truncate(trimmed_len);
    res
}

/// Pick ETA based on UTC month, day, hour and minute.
pub(crate) fn pick_eta(bv: &BitVec, index: usize) -> Result<Option<DateTime<Utc>>, ParseError> {
    pick_eta_with_now(bv, index, Utc.ymd(2000, 1, 1).and_hms(0, 0, 0))
}

/// Pick ETA based on UTC month, day, hour and minute. Define also 'now'. This function is needed
/// to make tests independent of the system time.
fn pick_eta_with_now(
    bv: &BitVec,
    index: usize,
    now: DateTime<Utc>,
) -> Result<Option<DateTime<Utc>>, ParseError> {
    // Pick ETA
    let mut month = pick_u64(bv, index, 4) as u32;
    let mut day = pick_u64(bv, index + 4, 5) as u32;
    let mut hour = pick_u64(bv, index + 4 + 5, 5) as u32;
    let mut minute = pick_u64(bv, index + 4 + 5 + 5, 6) as u32;

    // Check special case for no value
    if month == 0 && day == 0 && hour == 24 && minute == 60 {
        return Ok(None);
    }

    // Complete partially given datetime
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

    // Ensure that that params from nmea are parsable as valid date
    // Notice that we can't rely on ? operator here because of leap years
    let res_this = parse_valid_utc(now.year(), month, day, hour, minute, 30, 0);
    let res_next = parse_valid_utc(now.year() + 1, month, day, hour, minute, 30, 0);
    if res_this.is_err() && res_next.is_err() {
        // Both years result invalid date
        match res_this {
            Ok(_) => {
                unreachable!("This should never be reached");
            }
            Err(e) => Err(e),
        }
    } else if res_this.is_err() {
        // Only next year results valid date
        Ok(Some(res_next.unwrap()))
    } else if res_next.is_err() {
        // Only this year results valid date
        Ok(Some(res_this.unwrap()))
    } else {
        // Both years result a valid date
        // If the ETA is more than 180 days in past assume it's about next year
        let this_year_eta = res_this.unwrap();
        if now - Duration::days(180) <= this_year_eta {
            Ok(Some(this_year_eta))
        } else {
            Ok(res_next.ok())
        }
    }
}

/// Pick number field from a comma-separated sentence or `None` in case of an empty field.
pub(crate) fn pick_number_field<T: core::str::FromStr>(
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

/// Pick hex-formatted field from a comma-separated sentence or `None` in case of an empty field.
pub(crate) fn pick_hex_field<T: num_traits::Num>(
    split: &[&str],
    num: usize,
) -> Result<Option<T>, String> {
    split
        .get(num)
        .filter(|s| !s.is_empty())
        .map(|s| {
            T::from_str_radix(s, 16)
                .map_err(|_| format!("Failed to parse hex field {}: {}", num, s))
        })
        .transpose()
}

/// Pick field from a comma-separated sentence or `None` in case of an empty field.
pub(crate) fn pick_string_field(split: &[&str], num: usize) -> Option<String> {
    let s = split.get(num).unwrap_or(&"");
    if !s.is_empty() {
        Some(s.to_string())
    } else {
        None
    }
}

/// Parse time field of format HHMMSS and convert it to `DateTime<Utc>` using the current time.
pub(crate) fn parse_hhmmss(hhmmss: &str, now: DateTime<Utc>) -> Result<DateTime<Utc>, ParseError> {
    let (hour, minute, second) =
        parse_time(hhmmss).map_err(|_| format!("Invalid time format: {}", hhmmss))?;
    parse_valid_utc(now.year(), now.month(), now.day(), hour, minute, second, 0)
}

/// Parse time fields of formats YYMMDD and HHMMSS and convert them to `DateTime<Utc>`.
pub(crate) fn parse_yymmdd_hhmmss(yymmdd: &str, hhmmss: &str) -> Result<DateTime<Utc>, ParseError> {
    let now = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let century = (now.year() / 100) * 100;
    let (day, month, year) =
        parse_date(yymmdd).map_err(|_| format!("Invalid date format: {}", yymmdd))?;
    let (hour, minute, second) =
        parse_time(hhmmss).map_err(|_| format!("Invalid time format: {}", hhmmss))?;
    parse_valid_utc(century + year, month, day, hour, minute, second, 0)
}

/// Parse time field of format HHMMSS.SS and convert it to `DateTime<Utc>` using the given date.
pub(crate) fn parse_hhmmss_ss(
    hhmmss: &str,
    date: DateTime<Utc>,
) -> Result<DateTime<Utc>, ParseError> {
    let (hour, minute, second, nano) = parse_time_with_fractions(hhmmss)
        .map_err(|_| format!("Invalid time format: {}", hhmmss))?;
    parse_valid_utc(
        date.year(),
        date.month(),
        date.day(),
        hour,
        minute,
        second,
        nano,
    )
}

/// Pick date by picking the given field numbers. Set time part to midnight.
pub(crate) fn pick_date_with_fields(
    split: &[&str],
    year_field: usize,
    month_field: usize,
    day_field: usize,
    hour: u32,
    minute: u32,
    second: u32,
    nanos: u32,
) -> Result<DateTime<Utc>, ParseError> {
    let year = split.get(year_field).unwrap_or(&"").parse::<i32>()?;
    let month = split.get(month_field).unwrap_or(&"").parse::<u32>()?;
    let day = split.get(day_field).unwrap_or(&"").parse::<u32>()?;
    parse_valid_utc(year, month, day, hour, minute, second, nanos)
}

/// Pick time zone (`FixedOffset`) with the given field numbers.
pub(crate) fn pick_timezone_with_fields(
    split: &[&str],
    hour_field: usize,
    minute_field: usize,
) -> Result<FixedOffset, ParseError> {
    let hour = split.get(hour_field).unwrap_or(&"").parse::<i32>()?;
    let minute = split.get(minute_field).unwrap_or(&"0").parse::<i32>()?;

    if let Some(offset) = FixedOffset::east_opt(hour * 3600 + hour.signum() * minute * 60) {
        Ok(offset)
    } else {
        Err(ParseError::InvalidSentence(format!(
            "Time zone offset out of bounds: {}:{}",
            hour, minute
        )))
    }
}

/// Parse day, month and year from YYMMDD string.
fn parse_date(yymmdd: &str) -> Result<(u32, u32, i32), ParseError> {
    let day = pick_s2(yymmdd, 0).parse::<u32>()?;
    let month = pick_s2(yymmdd, 2).parse::<u32>()?;
    let year = pick_s2(yymmdd, 4).parse::<i32>()?;
    Ok((day, month, year))
}

/// Parse hour, minute and second from HHMMSS string.
fn parse_time(hhmmss: &str) -> Result<(u32, u32, u32), ParseError> {
    let hour = pick_s2(hhmmss, 0).parse::<u32>()?;
    let minute = pick_s2(hhmmss, 2).parse::<u32>()?;
    let second = pick_s2(hhmmss, 4).parse::<u32>()?;
    Ok((hour, minute, second))
}

/// Parse hour, minute, second and nano seconds from HHMMSS.SS string.
fn parse_time_with_fractions(hhmmss: &str) -> Result<(u32, u32, u32, u32), ParseError> {
    let hour = pick_s2(hhmmss, 0).parse::<u32>()?;
    let minute = pick_s2(hhmmss, 2).parse::<u32>()?;
    let second = pick_s2(hhmmss, 4).parse::<u32>()?;
    let nano = {
        let nano_str = hhmmss.get(6..).unwrap_or(".0");
        if !nano_str.is_empty() {
            (nano_str.parse::<f64>()? * 1000000000.0).round() as u32
        } else {
            0
        }
    };
    Ok((hour, minute, second, nano))
}

/// Parse Utc date from YYYY MM DD hh mm ss
pub(crate) fn parse_ymdhs(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
) -> Result<DateTime<Utc>, ParseError> {
    parse_valid_utc(year, month, day, hour, min, sec, 0)
}

/// Using _opt on Utc. Will catch invalid Date (ex: month > 12).
pub fn parse_valid_utc(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    nano: u32,
) -> Result<DateTime<Utc>, ParseError> {
    let opt_utc = Utc
        .ymd_opt(year, month, day)
        .and_hms_nano_opt(hour, min, sec, nano);
    match opt_utc {
        chrono::LocalResult::Single(valid_utc) | chrono::LocalResult::Ambiguous(valid_utc, _) => {
            Ok(valid_utc)
        }
        chrono::LocalResult::None => Err(format!(
            "Failed to parse Utc Date from y:{} m:{} d:{} h:{} m:{} s:{}",
            year, month, day, hour, min, sec
        )
        .into()),
    }
}

/// A simple helper to pick a substring of length two from the given string.
fn pick_s2(s: &str, i: usize) -> &str {
    let end = i + 2;
    s.get(i..end).unwrap_or("")
}

/// Parse latitude from two string.
/// Argument `lat_string` expects format DDMM.MMM representing latitude.
/// Argument `hemisphere` expects "N" for north or "S" for south. If `hemisphere` value
/// is something else, north is quietly used as a fallback.
pub(crate) fn parse_latitude_ddmm_mmm(
    lat_string: &str,
    hemisphere: &str,
) -> Result<Option<f64>, ParseError> {
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
        return Err(format!("Failed to parse latitude (DDMM.MMM) from {}", lat_string).into());
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
/// Argument `lon_string` expects format DDDMM.MMM representing longitude.
/// Argument `hemisphere` expects "E" for east or "W" for west. If `hemisphere` value is
/// something else, east is quietly used as a fallback.
pub(crate) fn parse_longitude_dddmm_mmm(
    lon_string: &str,
    hemisphere: &str,
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
    Ok(Some(match hemisphere {
        "E" => val,
        "W" => -val,
        _ => val,
    }))
}

/// Parse latitude from two string.
/// Argument `lat_string` expects a latitude offset in minutes
/// Argument `hemisphere` expects "N" for north or "S" for south. If `hemisphere` value
/// is something else, north is quietly used as a fallback.
pub(crate) fn parse_latitude_m_m(
    lat_string: &str,
    hemisphere: &str,
) -> Result<Option<f64>, ParseError> {
    if !lat_string.is_empty() {
        match lat_string.parse::<f64>() {
            Ok(lat) => match hemisphere {
                "N" => Ok(Some(lat / 60.0)),
                "S" => Ok(Some(-lat / 60.0)),
                _ => Err(format!("Bad hemispehre: {}", hemisphere).into()),
            },
            Err(_) => Err(format!("Failed to parse float: {}", lat_string).into()),
        }
    } else {
        Ok(None)
    }
}

/// Parse longitude from two string.
/// Argument `long_string` expects a longitude offset in minutes
/// Argument `hemisphere` expects "E" for east or "W" for west. If `hemisphere` value is
/// something else, east is quietly used as a fallback.
pub(crate) fn parse_longitude_m_m(
    lon_string: &str,
    hemisphere: &str,
) -> Result<Option<f64>, String> {
    if !lon_string.is_empty() {
        match lon_string.parse::<f64>() {
            Ok(lon) => match hemisphere {
                "E" => Ok(Some(lon / 60.0)),
                "W" => Ok(Some(-lon / 60.0)),
                _ => Err(format!("Bad hemispehre: {}", hemisphere)),
            },
            Err(_) => Err(format!("Failed to parse float: {}", lon_string)),
        }
    } else {
        Ok(None)
    }
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
    fn test_pick_eta() {
        // Valid case
        let bv = bitvec![
            1, 0, 1, 0, // 10
            0, 1, 0, 1, 1, // 11
            1, 0, 1, 1, 0, // 22
            1, 1, 1, 0, 0, 1, // 57
        ];
        let eta = pick_eta(&bv, 0).ok().unwrap();
        assert_eq!(
            eta,
            Some(Utc.ymd(eta.unwrap().year(), 10, 11).and_hms(22, 57, 30))
        );

        // Invalid month
        let bv = bitvec![
            1, 1, 0, 1, // 13
            0, 1, 0, 1, 1, // 11
            1, 0, 1, 1, 0, // 22
            1, 1, 1, 0, 0, 1, // 57
        ];
        assert_eq!(pick_eta(&bv, 0).is_ok(), false);

        // Invalid day
        let bv = bitvec![
            0, 0, 1, 0, // 2
            1, 1, 1, 1, 1, // 31
            1, 0, 1, 1, 0, // 22
            1, 1, 1, 0, 0, 1, // 57
        ];
        assert_eq!(pick_eta(&bv, 0).is_ok(), false);

        // Invalid hour
        let bv = bitvec![
            1, 0, 1, 0, // 10
            0, 1, 0, 1, 1, // 11
            1, 1, 0, 0, 1, // 25
            1, 1, 1, 0, 0, 1, // 57
        ];
        assert_eq!(pick_eta(&bv, 0).is_ok(), false);

        // Invalid minute
        let bv = bitvec![
            1, 0, 1, 0, // 10
            0, 1, 0, 1, 1, // 11
            1, 0, 1, 1, 0, // 22
            1, 1, 1, 1, 0, 1, // 61
        ];
        assert_eq!(pick_eta(&bv, 0).is_ok(), false);
    }

    #[test]
    fn test_pick_eta_with_now() {
        // February 28
        let feb28 = bitvec![
            0, 0, 1, 0, // 2
            1, 1, 1, 0, 0, // 28
            0, 0, 0, 0, 0, // 0
            0, 0, 0, 0, 0, 0, // 0
        ];

        //February 29
        let feb29 = bitvec![
            0, 0, 1, 0, // 2
            1, 1, 1, 0, 1, // 29
            0, 0, 0, 0, 0, // 0
            0, 0, 0, 0, 0, 0, // 0
        ];

        // Leap day case
        let then = Utc.ymd(2020, 12, 31).and_hms(0, 0, 0);
        assert_eq!(
            pick_eta_with_now(&feb29, 0, then).ok().unwrap(),
            Some(Utc.ymd(2020, 2, 29).and_hms(0, 0, 30))
        );

        // Non leap day case
        let then = Utc.ymd(2020, 12, 31).and_hms(0, 0, 0);
        assert_eq!(
            pick_eta_with_now(&feb28, 0, then).ok().unwrap(),
            Some(Utc.ymd(2021, 2, 28).and_hms(0, 0, 30))
        );

        // Non leap year invalid case
        let then = Utc.ymd(2021, 12, 31).and_hms(0, 0, 0);
        assert_eq!(pick_eta_with_now(&feb29, 0, then).is_ok(), false);

        // Non leap year valid case
        let then = Utc.ymd(2021, 12, 31).and_hms(0, 0, 0);
        assert_eq!(pick_eta_with_now(&feb28, 0, then).is_ok(), true);

        // One day late
        let then = Utc.ymd(2021, 3, 1).and_hms(0, 0, 0);
        assert_eq!(
            pick_eta_with_now(&feb28, 0, then).ok().unwrap(),
            Some(Utc.ymd(2021, 2, 28).and_hms(0, 0, 30))
        );

        // Six months late
        let then = Utc.ymd(2021, 8, 31).and_hms(0, 0, 0);
        assert_eq!(
            pick_eta_with_now(&feb28, 0, then).ok().unwrap(),
            Some(Utc.ymd(2022, 2, 28).and_hms(0, 0, 30))
        );
    }

    #[test]
    fn test_parse_valid_utc() {
        assert_eq!(parse_valid_utc(2020, 2, 29, 0, 0, 0, 0).is_ok(), true);
        assert_eq!(parse_valid_utc(2021, 2, 29, 0, 0, 0, 0).is_ok(), false);
    }

    #[test]
    fn test_pick_number_field() {
        let s: Vec<&str> = "128,0,8.0,,xyz".split(',').collect();
        assert_eq!(pick_number_field::<u8>(&s, 0).ok().unwrap().unwrap(), 128);
        assert_eq!(pick_number_field::<u8>(&s, 1).ok().unwrap().unwrap(), 0);
        assert_eq!(pick_number_field::<f64>(&s, 2).ok().unwrap().unwrap(), 8.0);
        assert_eq!(pick_number_field::<u16>(&s, 3).ok().unwrap(), None);
        assert_eq!(pick_number_field::<u32>(&s, 4).is_ok(), false);
        assert_eq!(pick_number_field::<u32>(&s, 5).ok().unwrap(), None);
    }

    #[test]
    fn test_pick_hex_field() {
        let s: Vec<&str> = "ff,0,,FFFF,8080808080808080".split(",").collect();
        assert_eq!(pick_hex_field::<u8>(&s, 0).unwrap().unwrap(), 255);
        assert_eq!(pick_hex_field::<u8>(&s, 1).unwrap().unwrap(), 0);
        assert_eq!(pick_hex_field::<u8>(&s, 2).unwrap(), None);
        assert_eq!(pick_hex_field::<u16>(&s, 3).unwrap().unwrap(), 65535);
        assert_eq!(
            pick_hex_field::<u64>(&s, 4).unwrap().unwrap(),
            9259542123273814144
        );
    }

    #[test]
    fn test_parse_latitude_m_m() {
        assert::close(
            parse_latitude_m_m("3480", "N").ok().unwrap().unwrap_or(0.0),
            58.0,
            0.1,
        );
        assert::close(
            parse_latitude_m_m("3480", "S").ok().unwrap().unwrap_or(0.0),
            -58.0,
            0.1,
        );
        assert_eq!(parse_latitude_m_m("3480", "X").is_ok(), false);
        assert_eq!(parse_latitude_m_m("ABCD", "N").is_ok(), false);
        assert_eq!(parse_latitude_m_m("", "N").is_ok(), true);
        assert_eq!(parse_latitude_m_m("", "N").ok().unwrap(), None);
    }

    #[test]
    fn test_parse_longitude_m_m() {
        assert::close(
            parse_longitude_m_m("1140", "E")
                .ok()
                .unwrap()
                .unwrap_or(0.0),
            19.0,
            0.1,
        );
        assert::close(
            parse_longitude_m_m("1140", "W")
                .ok()
                .unwrap()
                .unwrap_or(0.0),
            -19.0,
            0.1,
        );
        assert_eq!(parse_longitude_m_m("1140", "X").is_ok(), false);
        assert_eq!(parse_longitude_m_m("ABCD", "E").is_ok(), false);
        assert_eq!(parse_longitude_m_m("", "E").is_ok(), true);
        assert_eq!(parse_longitude_m_m("", "E").ok().unwrap(), None);
    }

    #[test]
    fn test_pick_string_field() {
        let s: Vec<&str> = "a,b,,dd,e".split(",").collect();
        assert_eq!(pick_string_field(&s, 0), Some("a".into()));
        assert_eq!(pick_string_field(&s, 1), Some("b".into()));
        assert_eq!(pick_string_field(&s, 2), None);
        assert_eq!(pick_string_field(&s, 3), Some("dd".into()));
        assert_eq!(pick_string_field(&s, 4), Some("e".into()));
        assert_eq!(pick_string_field(&s, 5), None);
    }

    #[test]
    fn test_parse_time_with_fractions() {
        assert_eq!(
            parse_time_with_fractions("123456.987").unwrap_or((0, 0, 0, 0)),
            (12, 34, 56, 987000000)
        );
        assert_eq!(
            parse_time_with_fractions("123456").unwrap_or((0, 0, 0, 0)),
            (12, 34, 56, 0)
        );
    }

    #[test]
    fn test_parse_hhmmss_ss() {
        // Valid case with fractions
        let then = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
        assert_eq!(
            parse_hhmmss_ss("123456.987", then).ok(),
            Some(Utc.ymd(2000, 1, 1).and_hms_nano(12, 34, 56, 987000000))
        );

        // Valid case without fractions
        let then = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
        assert_eq!(
            parse_hhmmss_ss("123456", then).ok(),
            Some(Utc.ymd(2000, 1, 1).and_hms_nano(12, 34, 56, 0))
        );

        // Invalid case
        assert_eq!(parse_hhmmss_ss("123456@", then).ok(), None);
    }

    #[test]
    fn test_pick_date_with_fields() {
        let s: Vec<&str> = "$GPZDA,072914.00,31,05,2018,+02,00".split(',').collect();
        assert_eq!(
            pick_date_with_fields(&s, 4, 3, 2, 0, 0, 0, 0).ok(),
            Some(Utc.ymd(2018, 5, 31).and_hms(0, 0, 0))
        )
    }

    #[test]
    fn test_pick_timezone_with_fields() {
        // Valid positive time zone
        let s: Vec<&str> = ",,,,,+4,30".split(',').collect();
        assert_eq!(
            pick_timezone_with_fields(&s, 5, 6).ok(),
            Some(FixedOffset::east(4 * 3600 + 30 * 60))
        );

        // Valid negative time zone
        let s: Vec<&str> = ",,,,,-4,30".split(',').collect();
        assert_eq!(
            pick_timezone_with_fields(&s, 5, 6).ok(),
            Some(FixedOffset::east(-4 * 3600 - 30 * 60))
        );

        // Invalid time zone
        let s: Vec<&str> = ",,,,,+25,00".split(',').collect();
        assert_eq!(pick_timezone_with_fields(&s, 5, 6).is_ok(), false);
    }
}
