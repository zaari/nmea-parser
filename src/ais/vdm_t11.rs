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

/// AIS VDM/VDO type 11: UTC/Date Response
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::UtcDateResponse(BaseStationReport {
        own_vessel: { own_vessel },
        station: { station },
        mmsi: { pick_u64(&bv, 8, 30) as u32 },
        timestamp: {
            Some(parse_ymdhs(
                pick_u64(&bv, 38, 14) as i32,
                pick_u64(&bv, 52, 4) as u32,
                pick_u64(&bv, 56, 5) as u32,
                pick_u64(&bv, 61, 5) as u32,
                pick_u64(&bv, 66, 6) as u32,
                pick_u64(&bv, 72, 6) as u32,
            )?)
        },
        high_position_accuracy: { pick_u64(&bv, 78, 1) != 0 },
        latitude: {
            let lat_raw = pick_i64(&bv, 107, 27) as i32;
            if lat_raw != 0x3412140 {
                Some((lat_raw as f64) / 600000.0)
            } else {
                None
            }
        },
        longitude: {
            let lon_raw = pick_i64(&bv, 79, 28) as i32;
            if lon_raw != 0x6791AC0 {
                Some((lon_raw as f64) / 600000.0)
            } else {
                None
            }
        },
        position_fix_type: {
            let raw = pick_u64(&bv, 134, 4) as u8;
            match raw {
                0 => None,
                _ => Some(PositionFixType::new(raw)),
            }
        },
        raim_flag: { pick_u64(&bv, 148, 1) != 0 },
        radio_status: { pick_u64(&bv, 149, 19) as u32 },
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type11() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,B,;4R33:1uUK2F`q?mOt@@GoQ00000,0*5D,s28089,d-103,T39.44353985,x147521,r08TPHI1,1242958962") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::UtcDateResponse(bsr) => {
                        assert_eq!(bsr.mmsi, 304137000);
                        assert_eq!(bsr.timestamp, Some(Utc.ymd(2009, 5, 22).and_hms(2, 22, 40)));
                        assert_eq!(bsr.high_position_accuracy, true);
                        assert::close(bsr.latitude.unwrap_or(0.0), 28.409, 0.001);
                        assert::close(bsr.longitude.unwrap_or(0.0), -94.407, 0.001);
                        assert_eq!(bsr.position_fix_type, Some(PositionFixType::GPS));
                        assert_eq!(bsr.raim_flag, false);
                        assert_eq!(bsr.radio_status, 0);
                    },
                    ParsedMessage::Incomplete => {
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
