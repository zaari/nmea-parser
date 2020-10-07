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

use chrono::prelude::*;

use regex::Regex;

#[doc(hidden)]
/// Make a key for storing NMEA sentence fragments
pub fn make_fragment_key(sentence_type: &String, message_id: u64, fragment_count: u8, fragment_number: u8, radio_channel_code: &str) -> String {
    format!("{},{},{},{},{}", sentence_type, fragment_count, fragment_number, message_id, radio_channel_code)
}

/// Converts AIVDM playload armored string into bit vector.
#[doc(hidden)]
pub fn parse_payload(payload: &String) -> Result<BitVec, String> {   
    // As described on page https://gpsd.gitlab.io/gpsd/AIVDM.html#_aivdmaivdo_payload_armoring
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

#[doc(hidden)]
/// Picks a numberic field from BitVec.
pub fn pick_u64(bv: &BitVec, index: usize, len: usize) -> u64 {
    let mut res = 0;
    for pos in index .. (index + len) {
        res = (res << 1) | (*bv.get(pos).unwrap_or(&false) as u64);
    }
    res
}

#[doc(hidden)]
/// Picks a signed numberic field from BitVec.
pub fn pick_i64(bv: &BitVec, index: usize, len: usize) -> i64 {
    let mut res = 0;
    for pos in index .. (index + len) {
        res = (res << 1) | (*bv.get(pos).unwrap_or(&false) as u64);
    }

    let sign_bit = 1 << (len - 1);
    if res & sign_bit != 0 {
        ((res & (sign_bit - 1)) as i64) - (sign_bit as i64)
    } else {
        res as i64
    }
}

#[doc(hidden)]
/// Pick a string from BitVec. Char_count is the length in characters. Character size is 6-bits.
pub fn pick_string(bv: &BitVec, index: usize, char_count: usize) -> String {
    let char_size = 6;
    let mut res = String::with_capacity(char_count);
    for i in 0 .. char_count {
        let ch = pick_u64(bv, index + i * char_size, char_size) as u32;
        assert!(ch < 64);
        if ch == 0 {
            break;
        } else if ch < 32 {
            res.push(std::char::from_u32(64 + ch).unwrap_or(' '))
        } else {
            res.push(std::char::from_u32(ch).unwrap_or(' '))
        }
    }
    res.trim_end().to_string()
}

#[doc(hidden)]
/// Pick ETA based on UTC month, day, hour and minute.
pub fn pick_eta(bv: &BitVec, index: usize) -> Option<DateTime::<Utc>> {
    let now = Utc::now().naive_utc();
    
    // Pick ETA
    let mut month  = pick_u64(bv, index, 4) as u32;
    let mut day    = pick_u64(bv, index + 4, 5) as u32;
    let mut hour   = pick_u64(bv, index + 4 + 5, 5) as u32;
    let mut minute = pick_u64(bv, index + 4 + 5 + 5, 6) as u32;
    let mut second = pick_u64(bv, index + 4 + 5 + 5 + 6, 6) as u32;
    
    if month == 0 && day == 0 && hour == 24 && minute == 60 && second == 60 {
        return None;
    }
    
    if month == 0  { month = now.month(); }
    if day == 0    { day = now.day(); }
    if hour == 24  { hour = 23; minute = 59; second = 59; }
    if minute == 60  { minute = 59; second = 59; }
    if second == 60  { second = 59; }
    
    // This and next year
    let this_year_eta = NaiveDate::from_ymd(now.year(), month, day).and_hms(hour, minute, second);
    let next_year_eta = NaiveDate::from_ymd(now.year(), month, day).and_hms(hour, minute, second);

    if now <= this_year_eta {
        Some(DateTime::<Utc>::from_utc(this_year_eta, Utc))
    } else {
        Some(DateTime::<Utc>::from_utc(next_year_eta, Utc))
    }
}

/// Pick field from comma-separated sentence or None if empty field.
pub fn pick_number_field<T: std::str::FromStr>(split: &Vec<&str>, num: usize) -> Result<Option<T>, String> {
    let s = split.get(num).unwrap_or(&"");
    if *s != "" {
        match s.parse::<T>() {
            Ok(p)   => { Ok(Some(p)) }, 
            Err(_e) => { Err(format!("Failed to parse field {}: {}", num, s)) }
        }
    } else {
        Ok(None)
    }
}

/// Parse time field of format HHMMSS and convert it to DateTime<Utc> using the current time.
pub fn parse_hhmmss(hhmmss: &str, now: DateTime<Utc>) -> Result<DateTime<Utc>, String> {
    if let Some(hour) = hhmmss[0..2].parse::<u32>().ok() {
        if let Some(minute) = hhmmss[2..4].parse::<u32>().ok() {
            if let Some(second) = hhmmss[4..6].parse::<u32>().ok() {
                return Ok(Utc.ymd(now.year(), now.month(), now.day()).and_hms(hour, minute, second))
            }
        }
    }
    return Err(format!("Invalid time format: {}", hhmmss));
}

/// Parse time fields of formats YYMMDD and HHMMSS and convert them to DateTime<Utc>.
pub fn parse_yymmdd_hhmmss(yymmdd: &str, hhmmss: &str) -> Result<DateTime<Utc>, String> {
    let century = (Utc::now().year() / 100) * 100;
    if let Some(day) = pick_s2(yymmdd, 0).parse::<u32>().ok() {
        if let Some(month) = pick_s2(yymmdd, 2).parse::<u32>().ok() {
            if let Some(year) = pick_s2(yymmdd, 4).parse::<i32>().ok() {
                if let Some(hour) = pick_s2(hhmmss, 0).parse::<u32>().ok() {
                    if let Some(minute) = pick_s2(hhmmss, 2).parse::<u32>().ok() {
                        if let Some(second) = pick_s2(hhmmss, 4).parse::<u32>().ok() {
                            return Ok(Utc.ymd(century + year, month, day)
                                         .and_hms(hour, minute, second))
                        }
                    }
                }
                return Err(format!("Invalid time format: {}", hhmmss));
            }
        }
    }
    return Err(format!("Invalid date format: {}", yymmdd));
}

/// A simple helper to pick a substring of length two from the given string.
fn pick_s2(s: &str, i: usize) -> String {
    s.chars().skip(i).take(2).collect()
}

/// Parse latitude from two string.
/// lat_string = DDMM.MMM representing latitude
/// hemisphere = N for north, S for south
pub fn parse_latitude_ddmm_mmm(lat_string: &str, hemisphere: &str) -> Result<Option<f64>, String> {
    // DDMM.MMM
    if lat_string != "" {
        let re = Regex::new(r"^([0-9][0-9])([0-9][0-9]\.[0-9]+)").unwrap();
        if let Some(caps) = re.captures(lat_string) {
            let d = caps.get(1).unwrap().as_str().parse::<f64>().ok().unwrap_or(0.0);
            let m = caps.get(2).unwrap().as_str().parse::<f64>().ok().unwrap_or(0.0);
            let val = d + m / 60.0;
            Ok(Some(match hemisphere { "N" => val , "S" => -val, _ => val }))
        } else {
            return Err(format!("Failed to parse latitude (DDMM.MMM) from {}", lat_string));
        }
    } else {
        Ok(None)
    }
}

/// Parse longitude from two string.
/// lon_string = DDDMM.MMM representing latitude
/// eastwest = E for north, W for south
pub fn parse_longitude_dddmm_mmm(lon_string: &str, eastwest: &str) -> Result<Option<f64>, String> {
    // DDDMM.MMM
    if lon_string != "" {
        let re = Regex::new(r"^([0-9][0-9][0-9])([0-9][0-9]\.[0-9]+)").unwrap();
        if let Some(caps) = re.captures(lon_string) {
            let d = caps.get(1).unwrap().as_str().parse::<f64>().ok().unwrap_or(0.0);
            let m = caps.get(2).unwrap().as_str().parse::<f64>().ok().unwrap_or(0.0);
            let val = d + m / 60.0;
            Ok(Some(match eastwest { "E" => val, "W" => -val, _ => val }))
        } else {
            return Err(format!("Failed to parse longitude (DDDMM.MMM) from {}", lon_string));
        }
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_payload() {
        match parse_payload(&"w7b0P1".to_string()) {
            Ok(bv) => {
                assert_eq!(bv, bits![1,1,1,1,1,1, 
                                     0,0,0,1,1,1, 
                                     1,0,1,0,1,0, 
                                     0,0,0,0,0,0, 
                                     1,0,0,0,0,0,
                                     0,0,0,0,0,1,
                                    ]);
            },
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

}
