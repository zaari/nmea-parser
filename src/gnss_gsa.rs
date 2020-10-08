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

#[doc(hidden)]
/// xxGSA: GPS DOP and active satellites 
pub fn handle(sentence: &str, nav_system: NavigationSystem) -> Result<ParsedSentence, String> {
    let split: Vec<&str> = sentence.split(',').collect();
    
    return Ok(ParsedSentence::Gsa(PositionPrecision{
        system:             nav_system,
        mode1_automatic: {
            let s = split.get(1).unwrap_or(&"");
            match s {
                &"M" => Some(false), &"A" => Some(true), &"" => None,
                _ => { return Err(format!("Invalid GPGSA mode: {}", s)); }
            }
        },
        mode2_3d: {
            let s = split.get(2).unwrap_or(&"");
            match s {
                &"1" => Some(FixMode::NotAvailable),
                &"2" => Some(FixMode::Fix2D),
                &"3" => Some(FixMode::Fix3D),
                &"" => None,
                _ => { return Err(format!("Invalid GPGSA fix type: {}", s)); }
            }
        },
        prn_numbers: {
            let mut v = Vec::with_capacity(12);
            for i in 3..15 {
                if split.get(i).unwrap_or(&"") != &"" {
                    if let Some(val) = pick_number_field(&split, i)? {
                        v.push(val);
                    }
                }
            }
            v
        },
        pdop: pick_number_field(&split, 15)?,
        hdop: pick_number_field(&split, 16)?,
        vdop: pick_number_field(&split, 17)?,
    }));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_gpgsa() {
        match parse_sentence("$GPGSA,A,3,19,28,14,18,27,22,31,39,,,,,1.7,1.0,1.3*34", 
                              &mut NmeaStore::new()) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Gsa(gsa) => {
                        assert_eq!(gsa.mode1_automatic, Some(true));
                        assert_eq!(gsa.mode2_3d, Some(FixMode::Fix3D));
                        assert_eq!(gsa.prn_numbers, vec![19,28,14,18,27,22,31,39]);
                        assert_eq!(gsa.pdop, Some(1.7));
                        assert_eq!(gsa.hdop, Some(1.0));
                        assert_eq!(gsa.vdop, Some(1.3));
                    },
                    ParsedSentence::Incomplete => {
                        assert!(false);
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            Err(e) => {
                assert_eq!(e, "OK");
            }
        }
    }
}
