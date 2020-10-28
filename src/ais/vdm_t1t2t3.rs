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

/// AIS VDM/VDO types 1-3: Position Report with SOTDMA/ITDMA
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedSentence, ParseError> {
    return Ok(ParsedSentence::VesselDynamicData(VesselDynamicData {
        own_vessel: { own_vessel },
        station: { station },
        ais_type: { AisClass::ClassA },
        mmsi: { pick_u64(&bv, 8, 30) as u32 },
        nav_status: { NavigationStatus::new(pick_u64(&bv, 38, 4) as u8) },
        rot: {
            let raw = pick_i64(&bv, 42, 8);
            if -126 <= raw && raw < 0 {
                Some(-((-raw as f64 * 708.0 / 126.0) / 4.733).powi(2))
            } else if 0 <= raw && raw <= 126 {
                Some(((raw as f64 * 708.0 / 126.0) / 4.733).powi(2))
            } else {
                None
            }
        },
        rot_direction: {
            let raw = pick_i64(&bv, 42, 8);
            if raw == -128 {
                None
            } else if raw <= -2 {
                Some(RotDirection::Port)
            } else if raw < 2 {
                Some(RotDirection::Center)
            } else if raw < 128 {
                Some(RotDirection::Starboard)
            } else {
                None
            }
        },
        sog_knots: {
            let raw = pick_u64(&bv, 50, 10);
            if raw < 1023 {
                Some((raw as f64) * 0.1)
            } else {
                None
            }
        },
        high_position_accuracy: pick_u64(&bv, 60, 1) != 0,
        latitude: {
            let lat_raw = pick_i64(&bv, 89, 27) as i32;
            if lat_raw != 0x3412140 {
                Some((lat_raw as f64) / 600000.0)
            } else {
                None
            }
        },
        longitude: {
            let lon_raw = pick_i64(&bv, 61, 28) as i32;
            if lon_raw != 0x6791AC0 {
                Some((lon_raw as f64) / 600000.0)
            } else {
                None
            }
        },
        cog: {
            let cog_raw = pick_u64(&bv, 116, 12);
            if cog_raw != 0xE10 {
                Some(cog_raw as f64 * 0.1)
            } else {
                None
            }
        },
        heading_true: {
            let th_raw = pick_u64(&bv, 128, 9);
            if th_raw != 511 {
                Some(th_raw as f64)
            } else {
                None
            }
        },
        timestamp_seconds: pick_u64(&bv, 137, 6) as u8,
        positioning_system_meta: {
            // second of UTC timestamp has some hidden information
            let sec_raw = pick_u64(&bv, 137, 6) as u16;
            match sec_raw {
                60 => None,
                61 => Some(PositioningSystemMeta::ManualInputMode),
                62 => Some(PositioningSystemMeta::DeadReckoningMode),
                63 => Some(PositioningSystemMeta::Inoperative),
                _ => Some(PositioningSystemMeta::Operative),
            }
        },
        current_gnss_position: { None },
        special_manoeuvre: {
            let raw = pick_u64(&bv, 143, 2);
            match raw {
                0 => None,
                1 => Some(true),
                2 => Some(true),
                _ => {
                    warn!("Unrecognized Maneuver Indicator value: {}", raw);
                    None
                }
            }
        },
        raim_flag: pick_u64(&bv, 148, 1) != 0,
        class_b_unit_flag: None,
        class_b_display: None,
        class_b_dsc: None,
        class_b_band_flag: None,
        class_b_msg22_flag: None,
        class_b_mode_flag: None,
        class_b_css_flag: None,
        radio_status: { Some(pick_u64(&bv, 149, 19) as u32) },
    }));
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type1() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,15RTgt0PAso;90TKcjM8h6g208CQ,0*4A") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselDynamicData(vdd) => {
                        assert_eq!(vdd.mmsi, 371798000);
                        assert_eq!(vdd.nav_status, NavigationStatus::UnderWayUsingEngine);
                        assert_eq!(vdd.rot, None);
                        assert_eq!(vdd.rot_direction, Some(RotDirection::Port));
                        assert_eq!(vdd.sog_knots, Some(12.3));
                        assert_eq!(vdd.high_position_accuracy, true);
                        assert_eq!((vdd.latitude.unwrap_or(0.0) * 10.0).round() as i32, 484); // 48.38163333333
                        assert_eq!((vdd.longitude.unwrap_or(0.0) * 10.0).round() as i32, -1234); // -123.395383333
                        assert_eq!(vdd.cog, Some(224.0));
                        assert_eq!(vdd.heading_true, Some(215.0));
                        assert_eq!(vdd.timestamp_seconds, 33);
                        assert_eq!(
                            vdd.positioning_system_meta,
                            Some(PositioningSystemMeta::Operative)
                        );
                        assert_eq!(vdd.special_manoeuvre, None);
                        assert_eq!(vdd.raim_flag, false);
                    }
                    ParsedSentence::Incomplete => {
                        assert!(false);
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }

    #[test]
    fn test_parse_vdm_type2() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,16SteH0P00Jt63hHaa6SagvJ087r,0*42") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselDynamicData(vdd) => {
                        assert_eq!(vdd.mmsi, 440348000);
                        assert_eq!(vdd.nav_status, NavigationStatus::UnderWayUsingEngine);
                        assert_eq!(vdd.rot, None);
                        assert_eq!(vdd.rot_direction, None);
                        assert_eq!(vdd.sog_knots, Some(0.0));
                        assert_eq!(vdd.high_position_accuracy, false);
                        assert_eq!((vdd.latitude.unwrap_or(0.0) * 10.0).round() as i32, 431); // 43.08015
                        assert_eq!((vdd.longitude.unwrap_or(0.0) * 10.0).round() as i32, -708); // -70.7582
                        assert_eq!(vdd.cog, Some(93.4));
                        assert_eq!(vdd.heading_true, None);
                        assert_eq!(vdd.timestamp_seconds, 13);
                        assert_eq!(
                            vdd.positioning_system_meta,
                            Some(PositioningSystemMeta::Operative)
                        );
                        assert_eq!(vdd.special_manoeuvre, None);
                        assert_eq!(vdd.raim_flag, false);
                    }
                    ParsedSentence::Incomplete => {
                        assert!(false);
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }

    #[test]
    fn test_parse_vdm_type3() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,38Id705000rRVJhE7cl9n;160000,0*40") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselDynamicData(vdd) => {
                        assert_eq!(vdd.mmsi, 563808000);
                        assert_eq!(vdd.nav_status, NavigationStatus::Moored);
                        assert_eq!(vdd.rot, Some(0.0));
                        assert_eq!(vdd.rot_direction, Some(RotDirection::Center));
                        assert_eq!(vdd.sog_knots, Some(0.0));
                        assert_eq!(vdd.high_position_accuracy, true);
                        assert::close(vdd.latitude.unwrap_or(0.0), 36.91, 0.01);
                        assert::close(vdd.longitude.unwrap_or(0.0), -76.33, 0.01);
                        assert_eq!(vdd.cog, Some(252.0));
                        assert_eq!(vdd.heading_true, Some(352.0));
                        assert_eq!(vdd.timestamp_seconds, 35);
                        assert_eq!(
                            vdd.positioning_system_meta,
                            Some(PositioningSystemMeta::Operative)
                        );
                        assert_eq!(vdd.special_manoeuvre, None);
                        assert_eq!(vdd.raim_flag, false);
                    }
                    ParsedSentence::Incomplete => {
                        assert!(false);
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
}
