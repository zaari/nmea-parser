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

/// Type 21: Aid-to-Navigation Report
#[derive(Default, Clone, Debug, PartialEq)]
pub struct AidToNavigationReport {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// Aid type (5 bits)
    pub aid_type: NavAidType,

    /// Name (120 bits)
    pub name: String,

    /// Position accuracy.
    high_position_accuracy: bool,

    /// Latitude
    pub latitude: Option<f64>,

    /// Longitude
    pub longitude: Option<f64>,

    /// Overall dimension / reference for position A (9 bits)
    pub dimension_to_bow: Option<u16>,
    /// Overall dimension / reference for position B (9 bits)
    pub dimension_to_stern: Option<u16>,
    /// Overall dimension / reference for position C (6 bits)
    pub dimension_to_port: Option<u16>,
    /// Overall dimension / reference for position C (6 bits)
    pub dimension_to_starboard: Option<u16>,

    // Type of electronic position fixing device.
    pub position_fix_type: Option<PositionFixType>,

    /// Derived from UTC second (6 bits)
    pub timestamp_seconds: u8,

    /// Off-position indicator (1 bit):
    /// true = off position, false = on position
    pub off_position_indicator: bool,

    /// Regional reserved, uninterpreted.
    pub regional: u8,

    /// Riverine And Inland Navigation systems blue sign:
    /// RAIM (Receiver autonomous integrity monitoring) flag of electronic position
    /// fixing device; false = RAIM not in use = default; true = RAIM in use
    pub raim_flag: bool,

    /// Virtual aid flag:
    /// true = virtual aid to navigation simulated by nearby AIS station
    /// false = real aid to navigation at indicated position
    pub virtual_aid_flag: bool,

    /// Assigned-mode flag
    pub assigned_mode_flag: bool,
}

impl LatLon for AidToNavigationReport {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

/// Type of navigation aid
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavAidType {
    /// Default, type not specified
    NotSpecified, // 0

    /// Reference point
    ReferencePoint, // 1

    /// RACON (radar transponder marking a navigation hazard)
    Racon, // 2

    /// Fixed structure off shore
    FixedStructure, // 3

    /// Reserved for future use.
    Reserved4, // 4

    /// Light without sectors
    LightWithoutSectors, // 5

    /// Light with sectors
    LightWithSectors, // 6

    /// Leading light front
    LeadingLightFront, // 7

    /// Leading light rear
    LeadingLightRear, // 8

    /// Beacon, Cardinal North
    BeaconCardinalNorth, // 9

    /// Beacon, Cardinal East
    BeaconCardinalEast, // 10

    /// Beacon, Cardinal South
    BeaconCardinalSouth, // 11

    /// Beacon, Cardinal West
    BeaconCardinalWest, // 12

    /// Beacon, Port
    BeaconLateralPort, // 13

    /// Beacon, Starboard
    BeaconLateralStarboard, // 14

    /// Beacon, preferred channel port
    BeaconLateralPreferredChannelPort, // 15

    /// Beacon, preferred channel starboard
    BeaconLateralPreferredChannelStarboard, // 16

    /// Beacon, isolated danger
    BeaconIsolatedDanger, // 17

    /// Beacon, safe water
    BeaconSafeWater, // 18

    /// Beacon, special mark
    BeaconSpecialMark, // 19

    /// Cardinal Mark, north
    CardinalMarkNorth, // 20

    /// Cardinal Mark, east
    CardinalMarkEast, // 21

    /// Cardinal Mark, south
    CardinalMarkSouth, // 22

    /// Cardinal Mark, west
    CardinalMarkWest, // 23

    /// Port hand mark
    PortHandMark, // 24

    /// Starboard hand mark
    StarboardHandMark, // 25

    /// Preferred channel, port
    PreferredChannelPort, // 26

    /// Preferred channel, starboard
    PreferredChannelStarboard, // 27

    /// Isolated danger
    IsolatedDanger, // 28

    /// Safe Water
    SafeWater, // 29

    /// Special mark
    SpecialMark, // 30

    /// Light vessel / LANBY / rigs
    LightVessel, // 31
}

impl NavAidType {
    fn new(raw: u8) -> Result<NavAidType, ParseError> {
        match raw {
            0 => Ok(NavAidType::NotSpecified),
            1 => Ok(NavAidType::ReferencePoint),
            2 => Ok(NavAidType::Racon),
            3 => Ok(NavAidType::FixedStructure),
            4 => Ok(NavAidType::Reserved4),
            5 => Ok(NavAidType::LightWithoutSectors),
            6 => Ok(NavAidType::LightWithSectors),
            7 => Ok(NavAidType::LeadingLightFront),
            8 => Ok(NavAidType::LeadingLightRear),
            9 => Ok(NavAidType::BeaconCardinalNorth),
            10 => Ok(NavAidType::BeaconCardinalEast),
            11 => Ok(NavAidType::BeaconCardinalSouth),
            12 => Ok(NavAidType::BeaconCardinalWest),
            13 => Ok(NavAidType::BeaconLateralPort),
            14 => Ok(NavAidType::BeaconLateralStarboard),
            15 => Ok(NavAidType::BeaconLateralPreferredChannelPort),
            16 => Ok(NavAidType::BeaconLateralPreferredChannelStarboard),
            17 => Ok(NavAidType::BeaconIsolatedDanger),
            18 => Ok(NavAidType::BeaconSafeWater),
            19 => Ok(NavAidType::BeaconSpecialMark),
            20 => Ok(NavAidType::CardinalMarkNorth),
            21 => Ok(NavAidType::CardinalMarkEast),
            22 => Ok(NavAidType::CardinalMarkSouth),
            23 => Ok(NavAidType::CardinalMarkWest),
            24 => Ok(NavAidType::PortHandMark),
            25 => Ok(NavAidType::StarboardHandMark),
            26 => Ok(NavAidType::PreferredChannelPort),
            27 => Ok(NavAidType::PreferredChannelStarboard),
            28 => Ok(NavAidType::IsolatedDanger),
            29 => Ok(NavAidType::SafeWater),
            30 => Ok(NavAidType::SpecialMark),
            31 => Ok(NavAidType::LightVessel),
            _ => Err(format!("Unrecognized Nav aid type code: {}", raw).into()),
        }
    }
}

impl Default for NavAidType {
    fn default() -> NavAidType {
        NavAidType::NotSpecified
    }
}

impl core::fmt::Display for NavAidType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NavAidType::NotSpecified => write!(f, "not specified"),
            NavAidType::ReferencePoint => write!(f, "reference point"),
            NavAidType::Racon => write!(f, "RACON"),
            NavAidType::FixedStructure => write!(f, "FixedStructure"),
            NavAidType::Reserved4 => write!(f, "(reserved)"),
            NavAidType::LightWithoutSectors => write!(f, "light without sectors"),
            NavAidType::LightWithSectors => write!(f, "light with sectors"),
            NavAidType::LeadingLightFront => write!(f, "leading light front"),
            NavAidType::LeadingLightRear => write!(f, "leading light rear"),
            NavAidType::BeaconCardinalNorth => write!(f, "cardinal beacon, north"),
            NavAidType::BeaconCardinalEast => write!(f, "cardinal beacon, east"),
            NavAidType::BeaconCardinalSouth => write!(f, "cardinal beacon, south"),
            NavAidType::BeaconCardinalWest => write!(f, "cardinal beacon, west"),
            NavAidType::BeaconLateralPort => write!(f, "lateral beacon, port side"),
            NavAidType::BeaconLateralStarboard => write!(f, "lateral beacon, starboard side"),
            NavAidType::BeaconLateralPreferredChannelPort => {
                write!(f, "lateral beacon, preferred channel, port side")
            }
            NavAidType::BeaconLateralPreferredChannelStarboard => {
                write!(f, "lateral beacon, preferred channel, starboard side")
            }
            NavAidType::BeaconIsolatedDanger => write!(f, "isolated danger beacon"),
            NavAidType::BeaconSafeWater => write!(f, "safe water"),
            NavAidType::BeaconSpecialMark => write!(f, "special mark"),
            NavAidType::CardinalMarkNorth => write!(f, "cardinal mark, north"),
            NavAidType::CardinalMarkEast => write!(f, "cardinal mark, east"),
            NavAidType::CardinalMarkSouth => write!(f, "cardinal mark, south"),
            NavAidType::CardinalMarkWest => write!(f, "cardinal mark, west"),
            NavAidType::PortHandMark => write!(f, "port hand mark"),
            NavAidType::StarboardHandMark => write!(f, "starboard hand mark"),
            NavAidType::PreferredChannelPort => write!(f, "preferred channel, port side"),
            NavAidType::PreferredChannelStarboard => write!(f, "preferred channel, starboard side"),
            NavAidType::IsolatedDanger => write!(f, "isolated danger"),
            NavAidType::SafeWater => write!(f, "safe water"),
            NavAidType::SpecialMark => write!(f, "special mark"),
            NavAidType::LightVessel => write!(f, "light vessel"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 21: Aid-to-Navigation Report
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::AidToNavigationReport(
        AidToNavigationReport {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            aid_type: {
                NavAidType::new(pick_u64(&bv, 38, 5) as u8)
                    .ok()
                    .unwrap_or(NavAidType::NotSpecified)
            },
            name: {
                let mut s = pick_string(&bv, 43, 20);
                s.push_str(&pick_string(&bv, 272, 14));
                s
            },
            high_position_accuracy: { pick_u64(&bv, 163, 1) != 0 },
            latitude: {
                let lat_raw = pick_i64(&bv, 192, 27) as i32;
                if lat_raw != 0x3412140 {
                    Some((lat_raw as f64) / 600000.0)
                } else {
                    None
                }
            },
            longitude: {
                let lon_raw = pick_i64(&bv, 164, 28) as i32;
                if lon_raw != 0x6791AC0 {
                    Some((lon_raw as f64) / 600000.0)
                } else {
                    None
                }
            },
            dimension_to_bow: { Some(pick_u64(&bv, 219, 9) as u16) },
            dimension_to_stern: { Some(pick_u64(&bv, 228, 9) as u16) },
            dimension_to_port: { Some(pick_u64(&bv, 237, 6) as u16) },
            dimension_to_starboard: { Some(pick_u64(&bv, 243, 6) as u16) },
            position_fix_type: { Some(PositionFixType::new(pick_u64(&bv, 249, 4) as u8)) },
            timestamp_seconds: { pick_u64(&bv, 253, 6) as u8 },
            off_position_indicator: { pick_u64(&bv, 243, 1) != 0 },
            regional: { pick_u64(&bv, 260, 8) as u8 },
            raim_flag: { pick_u64(&bv, 268, 1) != 0 },
            virtual_aid_flag: { pick_u64(&bv, 269, 1) != 0 },
            assigned_mode_flag: { pick_u64(&bv, 270, 1) != 0 },
        },
    ))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type21() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,2,1,5,B,E1mg=5J1T4W0h97aRh6ba84<h2d;W:Te=eLvH50```q,0*46") {
            Ok(ps) => match ps {
                ParsedMessage::Incomplete => {
                    assert!(true);
                }
                _ => {
                    assert!(false);
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }

        match p.parse_sentence("!AIVDM,2,2,5,B,:D44QDlp0C1DU00,2*36") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::AidToNavigationReport(atnr) => {
                        assert_eq!(atnr.mmsi, 123456789);
                        assert_eq!(atnr.aid_type, NavAidType::CardinalMarkNorth);
                        assert_eq!(atnr.name, "CHINA ROSE MURPHY EXPRESS ALERT");
                        assert_eq!(atnr.high_position_accuracy, false);
                        assert::close(atnr.latitude.unwrap_or(0.0), 47.9206183333, 0.00000001);
                        assert::close(atnr.longitude.unwrap_or(0.0), -122.698591667, 0.00000001);
                        assert_eq!(atnr.dimension_to_bow, Some(5));
                        assert_eq!(atnr.dimension_to_stern, Some(5));
                        assert_eq!(atnr.dimension_to_port, Some(5));
                        assert_eq!(atnr.dimension_to_starboard, Some(5));
                        assert_eq!(atnr.position_fix_type, Some(PositionFixType::GPS));
                        assert_eq!(atnr.timestamp_seconds, 50);
                        assert_eq!(atnr.off_position_indicator, false);
                        assert_eq!(atnr.regional, 165);
                        assert_eq!(atnr.raim_flag, false);
                        assert_eq!(atnr.virtual_aid_flag, false);
                        assert_eq!(atnr.assigned_mode_flag, false);
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
