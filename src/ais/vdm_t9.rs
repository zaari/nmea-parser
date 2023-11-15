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

/// Type 9: Standard SAR Aircraft Position Report
#[derive(Default, Clone, Debug, PartialEq)]
pub struct StandardSarAircraftPositionReport {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// Altitude
    pub altitude: Option<u16>,

    /// Speed over ground in knots. Value 1022 means 1022 knots or more.
    pub sog_knots: Option<u16>,

    /// Position accuracy: true = high (<= 10 m), false = low (> 10 m)
    pub high_position_accuracy: bool,

    /// Latitude
    pub latitude: Option<f64>,

    /// Longitude
    pub longitude: Option<f64>,

    /// Course over ground
    pub cog: Option<f64>,

    /// Derived from UTC second (6 bits)
    pub timestamp_seconds: u8,

    /// Regional, reserved.
    pub regional: u8,

    /// Data terminal ready:
    /// true = ready,
    /// false = not ready
    pub dte: bool,

    /// Assigned flag.
    pub assigned: bool,

    /// Riverine And Inland Navigation systems blue sign:
    /// RAIM (Receiver autonomous integrity monitoring) flag of electronic position
    /// fixing device; false = RAIM not in use = default; true = RAIM in use
    pub raim_flag: bool,

    /// Radio status (20 bits).
    pub radio_status: u32,
}

impl LatLon for StandardSarAircraftPositionReport {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 9: Standard SAR Aircraft Position Report
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::StandardSarAircraftPositionReport(
        StandardSarAircraftPositionReport {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(bv, 8, 30) as u32 },
            altitude: {
                let raw = pick_u64(bv, 38, 12) as u16;
                if raw != 4095 {
                    Some(raw)
                } else {
                    None
                }
            },
            sog_knots: {
                let raw = pick_u64(bv, 50, 10) as u16;
                if raw != 1023 {
                    Some(raw)
                } else {
                    None
                }
            },
            high_position_accuracy: { pick_u64(bv, 60, 1) != 0 },
            latitude: {
                let lat_raw = pick_i64(bv, 89, 27) as i32;
                if lat_raw != 0x3412140 {
                    Some((lat_raw as f64) / 600000.0)
                } else {
                    None
                }
            },
            longitude: {
                let lon_raw = pick_i64(bv, 61, 28) as i32;
                if lon_raw != 0x6791AC0 {
                    Some((lon_raw as f64) / 600000.0)
                } else {
                    None
                }
            },
            cog: {
                let cog_raw = pick_u64(bv, 116, 12);
                if cog_raw != 0xE10 {
                    Some(cog_raw as f64 * 0.1)
                } else {
                    None
                }
            },
            timestamp_seconds: pick_u64(bv, 128, 6) as u8,
            regional: { pick_u64(bv, 134, 8) as u8 },
            dte: { pick_u64(bv, 142, 1) == 0 },
            assigned: { pick_u64(bv, 146, 1) != 0 },
            raim_flag: { pick_u64(bv, 147, 1) != 0 },
            radio_status: { pick_u64(bv, 148, 20) as u32 },
        },
    ))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type9() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,B,91b55wi;hbOS@OdQAC062Ch2089h,0*30") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::StandardSarAircraftPositionReport(sapr) => {
                        assert_eq!(sapr.mmsi, 111232511);
                        assert_eq!(sapr.altitude, Some(303));
                        assert_eq!(sapr.sog_knots, Some(42));
                        assert!(!sapr.high_position_accuracy);
                        assert::close(sapr.longitude.unwrap_or(0.0), -6.27884, 0.00001);
                        assert::close(sapr.latitude.unwrap_or(0.0), 58.144, 0.00001);
                        assert_eq!(sapr.cog, Some(154.5));
                        assert_eq!(sapr.timestamp_seconds, 15);
                        assert_eq!(sapr.regional, 0);
                        assert!(!sapr.dte);
                        assert!(!sapr.assigned);
                        assert!(!sapr.raim_flag);
                        assert_eq!(sapr.radio_status, 33392);
                    }
                    ParsedMessage::Incomplete => {
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
