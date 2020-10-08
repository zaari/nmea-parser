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
/// xxGGA: Global Positioning System Fix Data
pub fn handle(sentence: &str, nav_system: NavigationSystem) -> Result<ParsedSentence, String> {
    let now: DateTime<Utc> = Utc::now();
    let split: Vec<&str> = sentence.split(',').collect();
    
    return Ok(ParsedSentence::Gga(PositionTimeSatelites{
        system:             nav_system,
        timestamp:          parse_hhmmss(split.get(1).unwrap_or(&""), now).ok(),
        latitude:           parse_latitude_ddmm_mmm(split.get(2).unwrap_or(&""), 
                                                    split.get(3).unwrap_or(&""))?,
        longitude:          parse_longitude_dddmm_mmm(split.get(4).unwrap_or(&""), 
                                                      split.get(5).unwrap_or(&""))?,
        quality:            GpsQualityIndicator::new(pick_number_field(&split, 6)?.unwrap_or(0)),
        satellite_count:    pick_number_field(&split, 7)?,
        hdop:               pick_number_field(&split, 8)?,
        altitude:           pick_number_field(&split, 9)?,
        geoid_separation:   pick_number_field(&split, 11)?,
        age_of_dgps:        pick_number_field(&split, 13)?,
        ref_station_id:     pick_number_field(&split, 14)?,
    }));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpgga() {
        match parse_sentence("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47", 
            &mut NmeaStore::new()) 
        {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Gga(gga) => {
                        assert_eq!(gga.timestamp, {
                            let now: DateTime<Utc> = Utc::now();
                            Some(Utc.ymd(now.year(), now.month(), now.day()).and_hms(12, 35, 19))
                        });
                        assert::close(gga.latitude.unwrap_or(0.0), 48.117, 0.001);
                        assert::close(gga.longitude.unwrap_or(0.0), 11.517, 0.001);
                        assert_eq!(gga.quality, GpsQualityIndicator::GpsFix);
                        assert_eq!(gga.satellite_count.unwrap_or(0), 8);
                        assert::close(gga.hdop.unwrap_or(0.0), 0.9, 0.1);
                        assert::close(gga.altitude.unwrap_or(0.0), 545.4, 0.1);
                        assert::close(gga.geoid_separation.unwrap_or(0.0), 46.9, 0.1);
                        assert_eq!(gga.age_of_dgps, None);
                        assert_eq!(gga.ref_station_id, None);
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
    fn test_parse_cpgga_southwest() {
        match parse_sentence("$GPGGA,123519,4807.0,S,01131.0,W,1,08,0.9,545.4,M,46.9,M,,", 
            &mut NmeaStore::new()) 
        {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Gga(gga) => {
                        assert_eq!((gga.latitude.unwrap_or(0.0) * 1000.0).round() as i32, -48117);
                        assert_eq!((gga.longitude.unwrap_or(0.0) * 1000.0).round() as i32, -11517);
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
    fn test_parse_cpgga_empty_fields() {
        match parse_sentence("$GPGGA,123519,,,,,,,,,,,,,*5B", &mut NmeaStore::new()) 
        {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Gga(gga) => {
                        assert_eq!(gga.timestamp, {
                            let now: DateTime<Utc> = Utc::now();
                            Some(Utc.ymd(now.year(), now.month(), now.day()).and_hms(12, 35, 19))
                        });
                        assert_eq!(gga.latitude, None);
                        assert_eq!(gga.longitude, None);
                        assert_eq!(gga.quality, GpsQualityIndicator::Invalid);
                        assert_eq!(gga.satellite_count, None);
                        assert_eq!(gga.hdop, None);
                        assert_eq!(gga.altitude, None);
                        assert_eq!(gga.geoid_separation, None);
                        assert_eq!(gga.age_of_dgps, None);
                        assert_eq!(gga.ref_station_id, None);
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

