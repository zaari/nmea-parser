/*
Copyright 2020 Timo Saarinen, Sebastian Urban

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

/// GNS - GNSS fix data
#[derive(Clone, Debug, PartialEq)]
pub struct GnsData {
    /// Navigation system
    pub source: NavigationSystem,

    /// UTC of position fix
    pub timestamp: Option<DateTime<Utc>>,

    /// Latitude in degrees
    pub latitude: Option<f64>,

    /// Longitude in degrees
    pub longitude: Option<f64>,

    /// GPS mode indicator
    pub gps_mode: GnsModeIndicator,

    /// GLONASS mode indicator
    pub glonass_mode: GnsModeIndicator,

    /// Mode indicators for other navigation systems
    pub other_modes: Vec<GnsModeIndicator>,

    /// Number of satellites in use
    pub satellite_count: Option<u8>,

    /// Horizontal dilution of position using all the satellites.
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

impl LatLon for GnsData {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

/// GNS mode indicator
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GnsModeIndicator {
    /// Satellite system not used in position fix, or fix not valid
    Invalid,
    /// Satellite system used in non-differential mode in position fix
    Autonomous,
    /// Satellite system used in differential mode in position fix
    Differential,
    /// Satellite system used in precision mode
    ///
    /// Precision mode is defined as:
    /// no deliberate degradation (such as Selective Availability) and
    /// higher resolution code (P-code) is used to compute position fix.
    Precise,
    /// Satellite system used in RTK mode with fixed integers
    RealTimeKinematic,
    /// Satellite system used in real time kinematic mode with floating integers
    RealTimeKinematicFloat,
    /// Estimated (dead reckoning) mode
    DeadReckoning,
    /// Manual input mode
    ManualInputMode,
    /// Simulator mode
    SimulationMode,
}

impl GnsModeIndicator {
    pub fn new(a: char) -> GnsModeIndicator {
        match a {
            'N' => GnsModeIndicator::Invalid,
            'A' => GnsModeIndicator::Autonomous,
            'D' => GnsModeIndicator::Differential,
            'P' => GnsModeIndicator::Precise,
            'R' => GnsModeIndicator::RealTimeKinematic,
            'F' => GnsModeIndicator::RealTimeKinematicFloat,
            'E' => GnsModeIndicator::DeadReckoning,
            'M' => GnsModeIndicator::ManualInputMode,
            'S' => GnsModeIndicator::SimulationMode,
            _ => GnsModeIndicator::Invalid,
        }
    }
}

impl core::fmt::Display for GnsModeIndicator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GnsModeIndicator::Invalid => write!(f, "invalid"),
            GnsModeIndicator::Autonomous => write!(f, "autonomous fix"),
            GnsModeIndicator::Differential => write!(f, "differential fix"),
            GnsModeIndicator::Precise => write!(f, "precise fix"),
            GnsModeIndicator::RealTimeKinematic => write!(f, "Real-Time Kinematic"),
            GnsModeIndicator::RealTimeKinematicFloat => {
                write!(f, "Real-Time Kinematic (floating point)")
            }
            GnsModeIndicator::DeadReckoning => write!(f, "dead reckoning"),
            GnsModeIndicator::ManualInputMode => write!(f, "manual input mode"),
            GnsModeIndicator::SimulationMode => write!(f, "simulation mode"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// xxGNS: Global Positioning System Fix Data
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let now: DateTime<Utc> = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let split: Vec<&str> = sentence.split(',').collect();
    let modes: Vec<char> = split.get(6).unwrap_or(&"").chars().collect();

    Ok(ParsedMessage::Gns(GnsData {
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
        gps_mode: GnsModeIndicator::new(*modes.get(0).unwrap_or(&' ')),
        glonass_mode: GnsModeIndicator::new(*modes.get(1).unwrap_or(&' ')),
        other_modes: modes
            .into_iter()
            .skip(2)
            .map(GnsModeIndicator::new)
            .collect(),
        satellite_count: pick_number_field(&split, 7)?,
        hdop: pick_number_field(&split, 8)?,
        altitude: pick_number_field(&split, 9)?,
        geoid_separation: pick_number_field(&split, 10)?,
        age_of_dgps: pick_number_field(&split, 11)?,
        ref_station_id: pick_number_field(&split, 12)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpgns() {
        // General test
        let mut p = NmeaParser::new();
        match p.parse_sentence(
            "$GNGNS,090310.00,4806.891632,N,01134.134167,E,AAN,10,1.0,532.4,47.0,,,V*68",
        ) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Gns(gns) => {
                        assert_eq!(gns.timestamp, {
                            Some(Utc.ymd(2000, 01, 01).and_hms(09, 03, 10))
                        });
                        assert::close(gns.latitude.unwrap_or(0.0), 48.114, 0.001);
                        assert::close(gns.longitude.unwrap_or(0.0), 11.569, 0.001);
                        assert_eq!(gns.gps_mode, GnsModeIndicator::Autonomous);
                        assert_eq!(gns.glonass_mode, GnsModeIndicator::Autonomous);
                        assert_eq!(gns.other_modes[0], GnsModeIndicator::Invalid);
                        assert_eq!(gns.satellite_count.unwrap_or(0), 10);
                        assert::close(gns.hdop.unwrap_or(0.0), 0.9, 0.1);
                        assert::close(gns.altitude.unwrap_or(0.0), 532.4, 0.1);
                        assert::close(gns.geoid_separation.unwrap_or(0.0), 47.0, 0.1);
                        assert_eq!(gns.age_of_dgps, None);
                        assert_eq!(gns.ref_station_id, None);
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
        match p.parse_sentence("$GPGNS,123519,,,,,,,,,,,,,*40") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Gns(gns) => {
                        assert_eq!(gns.timestamp, {
                            Some(Utc.ymd(2000, 1, 1).and_hms(12, 35, 19))
                        });
                        assert_eq!(gns.latitude, None);
                        assert_eq!(gns.longitude, None);
                        assert_eq!(gns.gps_mode, GnsModeIndicator::Invalid);
                        assert_eq!(gns.glonass_mode, GnsModeIndicator::Invalid);
                        assert!(gns.other_modes.is_empty());
                        assert_eq!(gns.satellite_count, None);
                        assert_eq!(gns.hdop, None);
                        assert_eq!(gns.altitude, None);
                        assert_eq!(gns.geoid_separation, None);
                        assert_eq!(gns.age_of_dgps, None);
                        assert_eq!(gns.ref_station_id, None);
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
