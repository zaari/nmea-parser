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

/// GSV - satellite information
#[derive(Clone, Debug, PartialEq)]
pub struct GsvData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Satellite PRN number
    pub prn_number: u8,
    
    /// Elevation in degrees (max 90°)
    pub elevation: Option<u8>,
    
    /// Azimuth in degrees from True north (0°-359°)
    pub azimuth: Option<u16>,
    
    /// SNR, 0-99 dB, None when not tracking
    pub snr: Option<u8>,
}

// -------------------------------------------------------------------------------------------------

#[doc(hidden)]
/// xxGSV: GPS Satellites in view
pub fn handle(sentence: &str, nav_system: NavigationSystem, store: &mut NmeaParser) 
              -> Result<ParsedSentence, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    let msg_type = split.get(0).unwrap_or(&"");
    let msg_count = pick_number_field(&split, 1)?.unwrap_or(0);
    let msg_num = pick_number_field(&split, 2)?.unwrap_or(0);
    store.push_string(make_gsv_key(msg_type, msg_count, msg_num), sentence.into());

    let mut found_count = 0;
    for i in 1..(msg_count + 1) {
        if store.contains_key(make_gsv_key(msg_type, msg_count, i)) {
            found_count += 1;
        }
    }

    if found_count == msg_count {
        let mut v = Vec::new();
        for i in 1..(msg_count + 1) {
            if let Some(sentence) = store.pull_string(make_gsv_key(msg_type, msg_count, i)) {
                let split: Vec<&str> = sentence.split(',').collect();
                for j in 0..4 {
                    if let Some(prn) = pick_number_field(&split, 4 + 4 * j as usize + 0).ok().unwrap_or(None) {
                        v.push(GsvData{
                            source: nav_system,
                            prn_number: prn,
                            elevation:  pick_number_field(&split, 4 + 4 * j as usize + 1).ok().unwrap_or(None),
                            azimuth:    pick_number_field(&split, 4 + 4 * j as usize + 2).ok().unwrap_or(None),
                            snr:        pick_number_field(&split, 4 + 4 * j as usize + 3).ok().unwrap_or(None),
                        });
                    }
                }
            }
        }
        
        return Ok(ParsedSentence::Gsv(v));
    } else {
        return Ok(ParsedSentence::Incomplete);
    }
}    

/// Make key for store
fn make_gsv_key(sentence_type: &str, msg_count: u32, msg_num: u32) -> String {
    format!("{},{},{}", sentence_type, msg_count, msg_num)
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

//    fn init() {
//        let _ = env_logger::builder().is_test(true).try_init();
//    }
    
    #[test]
    fn test_parse_cpgsv() {
        let mut p = NmeaParser::new();
        
        match p.parse_sentence("$GPGSV,3,1,11,03,03,111,00,04,15,270,00,06,01,010,00,13,06,292,00*74") 
        {
            Ok(ps) => { match ps { ParsedSentence::Incomplete => { }, _ => { assert!(false); } } },
            Err(e) => { assert_eq!(e.to_string(), "OK"); }
        }
        assert_eq!(p.strings_count(), 1);

        match p.parse_sentence("$GPGSV,3,2,11,14,25,170,00,16,57,208,39,18,67,296,40,19,40,246,00*74") 
        {
            Ok(ps) => { match ps { ParsedSentence::Incomplete => { }, _ => { assert!(false); } } },
            Err(e) => { assert_eq!(e.to_string(), "OK"); }
        }
        assert_eq!(p.strings_count(), 2);

        match p.parse_sentence("$GPGSV,3,3,11,22,42,067,42,24,14,311,43,27,05,244,00,,,,*4D") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Gsv(v) => {
                        assert_eq!(v.len(), 11);
                        
                        // 2nd satellite
                        let s2 = v.get(1).unwrap();
                        assert_eq!(s2.elevation, Some(15));
                        assert_eq!(s2.azimuth, Some(270));
                        assert_eq!(s2.snr, Some(0));
                        
                        // 5th satellite
                        let s5 = v.get(4).unwrap();
                        assert_eq!(s5.elevation, Some(25));
                        assert_eq!(s5.azimuth, Some(170));
                        assert_eq!(s5.snr, Some(0));
                        
                        // 11th satellite
                        let s11 = v.get(10).unwrap();
                        assert_eq!(s11.elevation, Some(5));
                        assert_eq!(s11.azimuth, Some(244));
                        assert_eq!(s11.snr, Some(0));
                    },
                    _ => {
                        assert_eq!(p.strings_count(), 3);
                        assert!(false);
                    }
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
        assert_eq!(p.strings_count(), 0);
    }
}

