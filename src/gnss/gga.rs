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

/// GGA - time, position, and fix related data
#[derive(Clone, Debug, PartialEq)]
pub struct GgaData {
    /// Navigation system
    pub source: NavigationSystem,

    /// UTC of position fix
    pub timestamp: Option<DateTime<Utc>>,

    /// Latitude in degrees
    pub latitude: Option<f64>,

    /// Longitude in degrees
    pub longitude: Option<f64>,

    /// GNSS Quality indicator
    pub quality: GgaQualityIndicator,

    /// Number of satellites in use
    pub satellite_count: Option<u8>,

    /// Horizontal dilution of position
    pub hdop: Option<f64>,

    /// Altitude above mean sea level (metres)
    pub altitude: Option<f64>,

    /// Height of geoid (mean sea level) above WGS84 ellipsoid
    pub geoid_separation: Option<f64>,

    /// Age of differential GPS data record, Type 1 or Type 9.
    pub age_of_dgps: Option<f64>,

    /// Reference station ID, range 0000-4095
    pub ref_station_id: Option<u16>,
}

impl LatLon for GgaData {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

/// GGA GPS quality indicator
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GgaQualityIndicator {
    Invalid,                // 0
    GpsFix,                 // 1
    DGpsFix,                // 2
    PpsFix,                 // 3
    RealTimeKinematic,      // 4
    RealTimeKinematicFloat, // 5
    DeadReckoning,          // 6
    ManualInputMode,        // 7
    SimulationMode,         // 8
}

impl GgaQualityIndicator {
    pub fn new(a: u8) -> GgaQualityIndicator {
        match a {
            0 => GgaQualityIndicator::Invalid,
            1 => GgaQualityIndicator::GpsFix,
            2 => GgaQualityIndicator::DGpsFix,
            3 => GgaQualityIndicator::PpsFix,
            4 => GgaQualityIndicator::RealTimeKinematic,
            5 => GgaQualityIndicator::RealTimeKinematicFloat,
            6 => GgaQualityIndicator::DeadReckoning,
            7 => GgaQualityIndicator::ManualInputMode,
            8 => GgaQualityIndicator::SimulationMode,
            _ => GgaQualityIndicator::Invalid,
        }
    }
}

impl core::fmt::Display for GgaQualityIndicator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GgaQualityIndicator::Invalid => write!(f, "invalid"),
            GgaQualityIndicator::GpsFix => write!(f, "GPS fix"),
            GgaQualityIndicator::DGpsFix => write!(f, "DGPS fix"),
            GgaQualityIndicator::PpsFix => write!(f, "PPS fix"),
            GgaQualityIndicator::RealTimeKinematic => write!(f, "Real-Time Kinematic"),
            GgaQualityIndicator::RealTimeKinematicFloat => {
                write!(f, "Real-Time Kinematic (floating point)")
            }
            GgaQualityIndicator::DeadReckoning => write!(f, "dead reckoning"),
            GgaQualityIndicator::ManualInputMode => write!(f, "manual input mode"),
            GgaQualityIndicator::SimulationMode => write!(f, "simulation mode"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// xxGGA: Global Positioning System Fix Data
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let now: DateTime<Utc> = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Gga(GgaData {
        source: nav_system,
        timestamp: parse_hhmmss(split.get(1).unwrap_or(&""), now).ok(),
        latitude: parse_latitude_ddmm_mmm(
            split.get(2).unwrap_or(&""),
            split.get(3).unwrap_or(&""),
        )?,
        longitude: parse_longitude_dddmm_mmm(
            split.get(4).unwrap_or(&""),
            split.get(5).unwrap_or(&""),
        )?,
        quality: GgaQualityIndicator::new(pick_number_field(&split, 6)?.unwrap_or(0)),
        satellite_count: pick_number_field(&split, 7)?,
        hdop: pick_number_field(&split, 8)?,
        altitude: pick_number_field(&split, 9)?,
        geoid_separation: pick_number_field(&split, 11)?,
        age_of_dgps: pick_number_field(&split, 13)?,
        ref_station_id: pick_number_field(&split, 14)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpgga() {
        // General test
        let mut p = NmeaParser::new();
        match p.parse_sentence("$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47")
        {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Gga(gga) => {
                        assert_eq!(gga.timestamp, {
                            Some(Utc.ymd(2000, 01, 01).and_hms(12, 35, 19))
                        });
                        assert::close(gga.latitude.unwrap_or(0.0), 48.117, 0.001);
                        assert::close(gga.longitude.unwrap_or(0.0), 11.517, 0.001);
                        assert_eq!(gga.quality, GgaQualityIndicator::GpsFix);
                        assert_eq!(gga.satellite_count.unwrap_or(0), 8);
                        assert::close(gga.hdop.unwrap_or(0.0), 0.9, 0.1);
                        assert::close(gga.altitude.unwrap_or(0.0), 545.4, 0.1);
                        assert::close(gga.geoid_separation.unwrap_or(0.0), 46.9, 0.1);
                        assert_eq!(gga.age_of_dgps, None);
                        assert_eq!(gga.ref_station_id, None);
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

        // Southwest test
        let mut p = NmeaParser::new();
        match p.parse_sentence("$GPGGA,123519,4807.0,S,01131.0,W,1,08,0.9,545.4,M,46.9,M,,") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Gga(gga) => {
                        assert_eq!(
                            (gga.latitude.unwrap_or(0.0) * 1000.0).round() as i32,
                            -48117
                        );
                        assert_eq!(
                            (gga.longitude.unwrap_or(0.0) * 1000.0).round() as i32,
                            -11517
                        );
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

        // Empty fields test
        let mut p = NmeaParser::new();
        match p.parse_sentence("$GPGGA,123519,,,,,,,,,,,,,*5B") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Gga(gga) => {
                        assert_eq!(gga.timestamp, {
                            Some(Utc.ymd(2000, 01, 01).and_hms(12, 35, 19))
                        });
                        assert_eq!(gga.latitude, None);
                        assert_eq!(gga.longitude, None);
                        assert_eq!(gga.quality, GgaQualityIndicator::Invalid);
                        assert_eq!(gga.satellite_count, None);
                        assert_eq!(gga.hdop, None);
                        assert_eq!(gga.altitude, None);
                        assert_eq!(gga.geoid_separation, None);
                        assert_eq!(gga.age_of_dgps, None);
                        assert_eq!(gga.ref_station_id, None);
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
