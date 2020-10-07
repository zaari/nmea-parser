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
/// AIVDM type 18: Standard Class B CS Position Report 
pub fn handle(bv: &BitVec, station: Station, own_vessel: bool) -> Result<ParsedSentence, String> {
    return Ok(ParsedSentence::VesselDynamicData(VesselDynamicData{
        own_vessel: {
            own_vessel
        },
        station: {
            station
        },
        ais_type: {
            AisClass::ClassB
        },
        mmsi: {
            pick_u64(&bv, 8, 30) as u32
        },
        sog_knots: {
                let raw = pick_u64(&bv, 46, 10);
                if raw < 1023 {
                    Some((raw as f64) * 0.1)
                } else {
                    None
                }
            },
        high_position_accuracy: pick_u64(&bv, 56, 1) != 0,
        longitude: {
                let lon_raw = pick_i64(&bv, 57, 28) as i32;
                if lon_raw != 0x6791AC0 {
                    Some((lon_raw as f64) / 600000.0)
                } else {
                    None
                }
            },
        latitude: {
                let lat_raw = pick_i64(&bv, 85, 27) as i32;
                if lat_raw != 0x3412140 {
                    Some((lat_raw as f64) / 600000.0)
                } else {
                    None
                }
            },
        cog: {
                let cog_raw = pick_u64(&bv, 112, 12);
                if cog_raw != 0xE10 {
                    Some(cog_raw as f64 * 0.1)
                } else {
                    None
                }
            },
        heading_true: {
                let th_raw = pick_u64(&bv, 124, 9);
                if th_raw != 511 {
                    Some(th_raw as f64)
                } else {
                    None
                }
            },
        timestamp_seconds: pick_u64(&bv, 133, 6) as u8,
        class_b_unit_flag: { 
            None 
        },
        class_b_display:    Some(pick_u64(&bv, 141, 1) != 0),
        class_b_dsc:        Some(pick_u64(&bv, 142, 1) != 0),
        class_b_band_flag:  Some(pick_u64(&bv, 143, 1) != 0),
        class_b_msg22_flag: Some(pick_u64(&bv, 144, 1) != 0),
        class_b_mode_flag:  Some(pick_u64(&bv, 145, 1) != 0),
        raim_flag:          pick_u64(&bv, 141, 1) != 0,
        class_b_css_flag: {
            None
        },
        radio_status: {
                pick_u64(&bv, 149, 19) as u32
            },
        nav_status:              NavigationStatus::NotDefined,
        rot:                     None,
        rot_direction:           None,
        positioning_system_meta: None,
        special_manoeuvre:       None,

    }));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_avidm_type18() {
        // https://fossies.org/linux/gpsd/test/sample.aivdm
        let s = "!AIVDM,1,1,,A,B52K>;h00Fc>jpUlNV@ikwpUoP06,0*4C";
        match decode_sentence(s, &mut NmeaStore::new()) {
            Ok(ps) => {
                match ps {
                   // The expected result
                    ParsedSentence::VesselDynamicData(vdd) => {
                        assert_eq!(vdd.mmsi, 338087471);
                        assert_eq!(vdd.nav_status, NavigationStatus::NotDefined);
                        assert_eq!(vdd.rot, None);
                        assert_eq!(vdd.rot_direction, None);
                        assert_eq!(vdd.sog_knots, Some(0.1));
                        assert_eq!(vdd.high_position_accuracy, false);
                        assert::close(vdd.latitude.unwrap_or(0.0), 40.7, 0.1);
                        assert::close(vdd.longitude.unwrap_or(0.0), -74.1, 0.1);
                        assert::close(vdd.cog.unwrap_or(0.0), 79.6, 0.1);
                        assert_eq!(vdd.heading_true, None);
                        assert_eq!(vdd.timestamp_seconds, 49);
                        assert_eq!(vdd.positioning_system_meta, None);
                        assert_eq!(vdd.special_manoeuvre, None);
                        assert_eq!(vdd.raim_flag, true);
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
