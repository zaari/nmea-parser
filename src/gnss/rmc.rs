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
/// xxRMC: Recommended minimum specific GPS/Transit data
pub fn handle(sentence: &str, nav_system: NavigationSystem) -> Result<ParsedSentence, String> {
    let split: Vec<&str> = sentence.split(',').collect();
    
    return Ok(ParsedSentence::Rmc(RmcData{
        system:             nav_system,
        timestamp:          parse_yymmdd_hhmmss(split.get(9).unwrap_or(&""), 
                                                split.get(1).unwrap_or(&"")).ok(),
        status_active:      {
            let s = split.get(2).unwrap_or(&"");
            match s {
                &"A" => Some(true),
                &"V" => Some(false),
                &"" => None,
                _ => { return Err(format!("Invalid RMC navigation receiver status: {}", s)); }
            }
        },
        latitude:           parse_latitude_ddmm_mmm(split.get(3).unwrap_or(&""), 
                                                    split.get(4).unwrap_or(&""))?,
        longitude:          parse_longitude_dddmm_mmm(split.get(5).unwrap_or(&""), 
                                                      split.get(6).unwrap_or(&""))?,
        speed_knots:        pick_number_field(&split, 7)?,
        bearing:            pick_number_field(&split, 8)?,
        variation: {
            if let Some(val) = pick_number_field::<f64>(&split, 10)? {
                let side = split.get(11).unwrap_or(&"");
                match side {
                    &"E" => { Some(val) },
                    &"W" => { Some(-val) },
                    _ => { return Err(format!("Invalid RMC variation side: {}", side)); },
                }
            } else {
                None
            }
        },
    }));

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cprmc() {
        match parse_sentence("$GPRMC,225446,A,4916.45,N,12311.12,W,000.5,054.7,191120,020.3,E*67", 
            &mut NmeaStore::new()) 
        {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Rmc(rmc) => {
                        assert_eq!(rmc.status_active, Some(true));
                        assert_eq!(rmc.timestamp, {
                            Some(Utc.ymd(2020, 11, 19).and_hms(22, 54, 46))
                        });
                        assert_eq!(rmc.speed_knots.unwrap(), 0.5);
                        assert::close(rmc.bearing.unwrap_or(0.0), 54.7, 0.1);
                        assert_eq!(rmc.variation.unwrap(), 20.3);
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

    #[test]
    fn test_parse_cprmc_empty_fields() {
        match parse_sentence("$GPRMC,225446,A,,,,,,,070809,,*23", &mut NmeaStore::new()) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Rmc(rmc) => {
                        assert_eq!(rmc.status_active, Some(true));
                        assert_eq!(rmc.timestamp, {
                            Some(Utc.ymd(2009, 8, 7).and_hms(22, 54, 46))
                        });
                        assert_eq!(rmc.speed_knots, None);
                        assert_eq!(rmc.bearing, None);
                        assert_eq!(rmc.variation, None);
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

