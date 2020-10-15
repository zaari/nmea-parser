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
/// AIS VDM/VDO type 27: Long Range AIS Broadcast message
pub fn handle(bv: &BitVec, station: Station, own_vessel: bool) -> Result<ParsedSentence, ParseError> {
    return Ok(ParsedSentence::VesselDynamicData(VesselDynamicData{
        own_vessel: {
            own_vessel
        },
        station: {
            station
        },
        ais_type: {
            AisClass::ClassA
        },
        mmsi: {
            pick_u64(&bv, 8, 30) as u32
        },
        nav_status: {
            NavigationStatus::new(pick_u64(&bv, 40, 4) as u8)
        },
        rot: {
            None
        },
        rot_direction: {
            None
        },
        sog_knots: {
            let sog_raw = pick_u64(&bv, 62, 6);
            if sog_raw != 63 {
                Some(sog_raw as f64)
            } else {
                None
            }
        },
        high_position_accuracy: {
            pick_u64(&bv, 38, 1) != 0
        },
        latitude: {
            let lat_raw = pick_i64(&bv, 44, 18) as i32;
            if lat_raw != 181000 {
                Some((lat_raw as f64) / 600.0) 
            } else {
                None
            }
        },
        longitude: {
            let lon_raw = pick_i64(&bv, 62, 17) as i32;
            if lon_raw != 181000 {
                Some((lon_raw as f64) / 600.0)
            } else {
                None
            }
        },
        cog: {
            let cog_raw = pick_u64(&bv, 62, 17);
            if cog_raw !=  91000 {
                Some(cog_raw as f64 * 0.1)
            } else {
                None
            }
        },
        heading_true:               None,
        timestamp_seconds:          0,
        positioning_system_meta:    None,
        current_gnss_position:      Some(pick_u64(&bv, 62, 1) == 0),
        special_manoeuvre:          None,
        raim_flag:                  pick_u64(&bv, 39, 1) != 0,
        class_b_unit_flag:          None,
        class_b_display:            None,
        class_b_dsc:                None,
        class_b_band_flag:          None,
        class_b_msg22_flag:         None,
        class_b_mode_flag:          None,
        class_b_css_flag:           None,
        radio_status:               None,
    }));
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type27() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,B,KC5E2b@U19PFdLbMuc5=ROv62<7m,0*16") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselDynamicData(vdd) => {
                        assert_eq!(vdd.mmsi, 206914217);
                        assert_eq!(vdd.nav_status, NavigationStatus::NotUnderCommand);
                        assert_eq!(vdd.rot, None);
                        assert_eq!(vdd.rot_direction, None);
                        assert_eq!(vdd.sog_knots, Some(1.0));
                        assert_eq!(vdd.high_position_accuracy, false);
                        assert::close(vdd.latitude.unwrap_or(0.0), 137.0, 0.1);
                        assert::close(vdd.longitude.unwrap_or(0.0),  4.8, 0.1);
                        assert::close(vdd.cog.unwrap_or(0.0), 290.0, 1.0);
                        assert_eq!(vdd.timestamp_seconds, 0);
                        assert_eq!(vdd.current_gnss_position, Some(true));
                        assert_eq!(vdd.raim_flag, false);
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
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
}

