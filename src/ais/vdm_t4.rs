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

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 4: Base Station Report
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BaseStationReport { 
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// Timestamp
    pub timestamp: Option<DateTime<Utc>>,

    /// Position accuracy: true = high (<= 10 m), false = low (> 10 m)
    pub high_position_accuracy: bool,

    /// Latitude
    pub latitude: Option<f64>,

    /// Longitude
    pub longitude: Option<f64>,
    
    // Type of electronic position fixing device.
    pub position_fix_type: Option<PositionFixType>,

    /// Riverine And Inland Navigation systems blue sign:
    /// RAIM (Receiver autonomous integrity monitoring) flag of electronic position 
    /// fixing device; false = RAIM not in use = default; true = RAIM in use
    pub raim_flag: bool,

    /// Communication state
    /// Diagnostic information for the radio system. 
    /// https://www.itu.int/dms_pubrec/itu-r/rec/m/R-REC-M.1371-1-200108-S!!PDF-E.pdf
    pub radio_status: u32,
}

impl LatLon for BaseStationReport {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

// -------------------------------------------------------------------------------------------------

#[doc(hidden)]
/// AIS VDM/VDO types 4: Base Station Report
pub fn handle(bv: &BitVec, station: Station, own_vessel: bool) -> Result<ParsedSentence, ParseError> {
    return Ok(ParsedSentence::BaseStationReport(BaseStationReport{
        own_vessel: {
            own_vessel
        },
        station: {
            station
        },
        mmsi: {
            pick_u64(&bv, 8, 30) as u32
        },
        timestamp: {
            Some(Utc.ymd(pick_u64(&bv, 38, 14) as i32, 
                        pick_u64(&bv, 52, 4) as u32, 
                        pick_u64(&bv, 56, 5) as u32)
                    .and_hms(pick_u64(&bv, 61, 5) as u32, 
                             pick_u64(&bv, 66, 6) as u32, 
                             pick_u64(&bv, 72, 6) as u32))
        },        
        high_position_accuracy: {
            pick_u64(&bv, 78, 1) != 0
        },
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
                    0 => { None },
                    _ => { Some(PositionFixType::new(raw)) },
                }
            },
        raim_flag: {
                pick_u64(&bv, 148, 1) != 0
            },
        radio_status: {
                pick_u64(&bv, 149, 19) as u32
            },
    }));
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type4() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,403OviQuMGCqWrRO9>E6fE700@GO,0*4D") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::BaseStationReport(bsr) => {
                        assert_eq!(bsr.mmsi, 3669702);
                        assert_eq!(bsr.timestamp, Some(Utc.ymd(2007, 5, 14).and_hms(19, 57, 39)));
                        assert_eq!(bsr.high_position_accuracy, true);
                        assert::close(bsr.latitude.unwrap_or(0.0), 36.884, 0.001);
                        assert::close(bsr.longitude.unwrap_or(0.0), -76.352, 0.001);
                        assert_eq!(bsr.position_fix_type, Some(PositionFixType::Surveyed));
                        assert_eq!(bsr.raim_flag, false);
                        assert_eq!(bsr.radio_status, 67039);
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

