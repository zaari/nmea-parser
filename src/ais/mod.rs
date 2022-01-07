/*
Copyright 2020-2021 Timo Saarinen

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

//! AIS VDM/VDO data structures

pub(crate) mod vdm_t1t2t3;
pub(crate) mod vdm_t4;
pub(crate) mod vdm_t5;
pub(crate) mod vdm_t6;
pub(crate) mod vdm_t9;
pub(crate) mod vdm_t10;
pub(crate) mod vdm_t11;
pub(crate) mod vdm_t12;
pub(crate) mod vdm_t13;
pub(crate) mod vdm_t14;
pub(crate) mod vdm_t15;
pub(crate) mod vdm_t16;
pub(crate) mod vdm_t17;
pub(crate) mod vdm_t18;
pub(crate) mod vdm_t19;
pub(crate) mod vdm_t20;
pub(crate) mod vdm_t21;
pub(crate) mod vdm_t22;
pub(crate) mod vdm_t23;
pub(crate) mod vdm_t24;
pub(crate) mod vdm_t25;
pub(crate) mod vdm_t26;
pub(crate) mod vdm_t27;

use super::*;
pub use vdm_t4::BaseStationReport;
pub use vdm_t6::BinaryAddressedMessage;
pub use vdm_t9::StandardSarAircraftPositionReport;
pub use vdm_t10::UtcDateInquiry;
pub use vdm_t12::AddressedSafetyRelatedMessage;
pub use vdm_t13::SafetyRelatedAcknowledgement;
pub use vdm_t14::SafetyRelatedBroadcastMessage;
pub use vdm_t15::{Interrogation, InterrogationCase};
pub use vdm_t16::AssignmentModeCommand;
pub use vdm_t17::DgnssBroadcastBinaryMessage;
pub use vdm_t20::{DataLinkManagementMessage};
pub use vdm_t21::{AidToNavigationReport, NavAidType};
pub use vdm_t22::{ChannelManagement};
pub use vdm_t23::{GroupAssignmentCommand};
pub use vdm_t25::{SingleSlotBinaryMessage};
pub use vdm_t26::{MultipleSlotBinaryMessage};

// -------------------------------------------------------------------------------------------------

/// AIS station based on talker id
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Station {
    BaseStation,             // !AB
    DependentAisBaseStation, // !AD
    MobileStation,           // !AI (the most common one)
    AidToNavigationStation,  // !AN
    AisReceivingStation,     // !AR
    LimitedBaseStation,      // !AS
    AisTransmittingStation,  // !AT
    RepeaterStation,         // !AX
    Other,                   // !BS, !SA, etc.
}

impl Default for Station {
    fn default() -> Station {
        Station::Other
    }
}

impl core::fmt::Display for Station {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Station::BaseStation => write!(f, "base station"),
            Station::DependentAisBaseStation => write!(f, "dependent AIS base station"),
            Station::MobileStation => write!(f, "mobile station"),
            Station::AidToNavigationStation => write!(f, "aid to navigation station"),
            Station::AisReceivingStation => write!(f, "ais receiving station"),
            Station::LimitedBaseStation => write!(f, "limited base station"),
            Station::AisTransmittingStation => write!(f, "AIS transmitting station"),
            Station::RepeaterStation => write!(f, "repeater station"),
            Station::Other => write!(f, "other"),
        }
    }
}

impl core::str::FromStr for Station {
    type Err = ParseError;

    fn from_str(talker_id: &str) -> Result<Self, Self::Err> {
        if talker_id.len() < 2 {
            return Err(ParseError::InvalidSentence(
                "Invalid station identifier".to_string(),
            ));
        }
        match &talker_id[0..2] {
            "AB" => Ok(Self::BaseStation),
            "AD" => Ok(Self::DependentAisBaseStation),
            "AI" => Ok(Self::MobileStation),
            "AN" => Ok(Self::AidToNavigationStation),
            "AR" => Ok(Self::AisReceivingStation),
            "AS" => Ok(Self::LimitedBaseStation),
            "AT" => Ok(Self::AisTransmittingStation),
            "AX" => Ok(Self::RepeaterStation),
            _ => Ok(Self::Other),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Types 1, 2, 3 and 18: Position Report Class A, and Long Range AIS Broadcast message
#[derive(Default, Clone, Debug, PartialEq)]
pub struct VesselDynamicData {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Class A or Class B
    pub ais_type: AisClass,

    /// User ID (30 bits)
    pub mmsi: u32,

    // TODO: timestamp
    /// Navigation status
    pub nav_status: NavigationStatus,

    /// Accurate ROT_sensor (±0..708°/min) if available.
    pub rot: Option<f64>,

    /// ROT direction when turn is more than 5°/30s.
    pub rot_direction: Option<RotDirection>,

    /// Speed over ground in knots
    pub sog_knots: Option<f64>,

    /// Position accuracy: true = high (<= 10 m), false = low (> 10 m)
    pub high_position_accuracy: bool,

    /// Latitude
    pub latitude: Option<f64>,

    /// Longitude
    pub longitude: Option<f64>,

    /// Course over ground
    pub cog: Option<f64>,

    /// True heading (0-359)
    pub heading_true: Option<f64>,

    /// Derived from UTC second (6 bits)
    pub timestamp_seconds: u8,

    /// Positioning system metadata (included in seconds in UTC timestamp)
    pub positioning_system_meta: Option<PositioningSystemMeta>,

    /// GNSS position status (Type 27):
    ///  true = current GNSS position
    ///  false = not GNSS position
    pub current_gnss_position: Option<bool>,

    /// Special manoeuvre indicator. false = not engaged in special manoeuvre,
    /// true = engaged in special manouvre.
    pub special_manoeuvre: Option<bool>,

    /// Riverine And Inland Navigation systems blue sign:
    /// RAIM (Receiver autonomous integrity monitoring) flag of electronic position
    /// fixing device; false = RAIM not in use = default; true = RAIM in use
    pub raim_flag: bool,

    /// Class B unit flag: false = Class B SOTDMA unit, true = Class B "CS" unit.
    pub class_b_unit_flag: Option<bool>,

    /// Class B display:
    /// false = No display available; not capable of displaying Message 12 and 14
    /// true  = Equipped with integrated display displaying Message 12 and 14
    pub class_b_display: Option<bool>,

    /// Class B DSC:
    /// false = Not equipped with DSC function
    /// true  = Equipped with DSC function (dedicated or time-shared
    pub class_b_dsc: Option<bool>,

    /// Class B band flag:
    /// false = Capable of operating over the upper 525 kHz band of the marine band
    /// true  = Capable of operating over the whole marine band (irrelevant if
    ///         “Class B Message 22 flag” is 0)
    pub class_b_band_flag: Option<bool>,

    /// Class B Message 22 flag:
    /// false = No frequency management via Message 22 , operating on AIS1, AIS2 only
    /// true  = Frequency management via Message 22
    pub class_b_msg22_flag: Option<bool>,

    /// Mode flag:
    /// false = Station operating in autonomous and continuous mode = default
    /// true  = Station operating in assigned mode
    pub class_b_mode_flag: Option<bool>,

    /// Communication state selector flag
    /// false = SOTDMA communication state follows
    /// true  = ITDMA communication state follows (always “1” for Class-B “CS”)
    pub class_b_css_flag: Option<bool>,

    /// Communication state
    /// Diagnostic information for the radio system.
    /// https://www.itu.int/dms_pubrec/itu-r/rec/m/R-REC-M.1371-1-200108-S!!PDF-E.pdf
    pub radio_status: Option<u32>,
}

/// AIS class which is either Class A or Class B
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AisClass {
    /// AIS class not known.
    Unknown,

    /// AIS class A.
    ClassA, // Message types 1, 2, 3, 5

    /// AIS class B.
    ClassB, // Message types 14, 18, 19, 24
}

impl Default for AisClass {
    fn default() -> AisClass {
        AisClass::Unknown
    }
}

impl core::fmt::Display for AisClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AisClass::Unknown => write!(f, "unknown"),
            AisClass::ClassA => write!(f, "Class A"),
            AisClass::ClassB => write!(f, "Class B"),
        }
    }
}

impl LatLon for VesselDynamicData {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

/// Navigation status for VesselDynamicData
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavigationStatus {
    UnderWayUsingEngine = 0,        // 0
    AtAnchor = 1,                   // 1
    NotUnderCommand = 2,            // 2
    RestrictedManoeuverability = 3, // 3
    ConstrainedByDraught = 4,       // 4
    Moored = 5,                     // 5
    Aground = 6,                    // 6
    EngagedInFishing = 7,           // 7
    UnderWaySailing = 8,            // 8
    Reserved9 = 9,                  // 9, may be renamed in the future
    Reserved10 = 10,                // 10, may be renamed in the future
    Reserved11 = 11,                // 11, may be renamed in the future
    Reserved12 = 12,                // 12, may be renamed in the future
    Reserved13 = 13,                // 13, may be renamed in the future
    AisSartIsActive = 14,           // 14
    NotDefined = 15,                // 15
}
impl NavigationStatus {
    pub fn new(nav_status: u8) -> NavigationStatus {
        match nav_status {
            0 => NavigationStatus::UnderWayUsingEngine,
            1 => NavigationStatus::AtAnchor,
            2 => NavigationStatus::NotUnderCommand,
            3 => NavigationStatus::RestrictedManoeuverability,
            4 => NavigationStatus::ConstrainedByDraught,
            5 => NavigationStatus::Moored,
            6 => NavigationStatus::Aground,
            7 => NavigationStatus::EngagedInFishing,
            8 => NavigationStatus::UnderWaySailing,
            9 => NavigationStatus::Reserved9,
            10 => NavigationStatus::Reserved10,
            11 => NavigationStatus::Reserved11,
            12 => NavigationStatus::Reserved12,
            13 => NavigationStatus::Reserved13,
            14 => NavigationStatus::AisSartIsActive,
            15 => NavigationStatus::NotDefined,
            _ => NavigationStatus::NotDefined,
        }
    }

    pub fn to_value(&self) -> u8 {
        *self as u8
    }
}

impl core::fmt::Display for NavigationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NavigationStatus::UnderWayUsingEngine => write!(f, "under way using engine"),
            NavigationStatus::AtAnchor => write!(f, "at anchor"),
            NavigationStatus::NotUnderCommand => write!(f, "not under command"),
            NavigationStatus::RestrictedManoeuverability => {
                write!(f, "restricted manoeuverability")
            }
            NavigationStatus::ConstrainedByDraught => write!(f, "constrained by draught"),
            NavigationStatus::Moored => write!(f, "moored"),
            NavigationStatus::Aground => write!(f, "aground"),
            NavigationStatus::EngagedInFishing => write!(f, "engaged in fishing"),
            NavigationStatus::UnderWaySailing => write!(f, "under way sailing"),
            NavigationStatus::Reserved9 => write!(f, "(reserved9)"),
            NavigationStatus::Reserved10 => write!(f, "(reserved10)"),
            NavigationStatus::Reserved11 => write!(f, "(reserved11)"),
            NavigationStatus::Reserved12 => write!(f, "(reserved12)"),
            NavigationStatus::Reserved13 => write!(f, "(reserved13)"),
            NavigationStatus::AisSartIsActive => write!(f, "ais sart is active"),
            NavigationStatus::NotDefined => write!(f, "(notDefined)"),
        }
    }
}

impl Default for NavigationStatus {
    fn default() -> NavigationStatus {
        NavigationStatus::NotDefined
    }
}

// -------------------------------------------------------------------------------------------------

/// Location metadata about positioning system
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositioningSystemMeta {
    Operative, // When timestamp second is 0-59
    ManualInputMode,
    DeadReckoningMode,
    Inoperative,
}

impl core::fmt::Display for PositioningSystemMeta {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PositioningSystemMeta::Operative => write!(f, "operative"),
            PositioningSystemMeta::ManualInputMode => write!(f, "manual input mode"),
            PositioningSystemMeta::DeadReckoningMode => write!(f, "dead reckoning mode"),
            PositioningSystemMeta::Inoperative => write!(f, "inoperative"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Vessel rotation direction
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RotDirection {
    /// Turning port (left, when seen by an observer aboard the vessel looking forward)
    Port,

    /// Not turning
    Center,

    /// Turning starboard (right, when seen by an observer aboard the vessel looking forward)
    Starboard,
}

impl Default for RotDirection {
    fn default() -> RotDirection {
        RotDirection::Center
    }
}

impl core::fmt::Display for RotDirection {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RotDirection::Port => write!(f, "port"),
            RotDirection::Center => write!(f, "center"),
            RotDirection::Starboard => write!(f, "starboard"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Types 5 and 24: Ship static voyage related data, and boat static data report.
#[derive(Default, Clone, Debug, PartialEq)]
pub struct VesselStaticData {
    /// True if the data is about own vessel, false if about other vessel.
    pub own_vessel: bool,

    /// Class A or Class B
    pub ais_type: AisClass,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// AIS version indicator (2 bits)
    pub ais_version_indicator: u8,

    /// IMO number (1-999999999; 30 bits).
    pub imo_number: Option<u32>,

    /// Call sign (7 ASCII characters)
    pub call_sign: Option<String>,

    /// Name (20 ASCII characters)
    pub name: Option<String>,

    /// Type of ship (first 4 of 8 bits)
    pub ship_type: ShipType,

    /// Type of ship and cargo (last 4 of 8 bits)
    pub cargo_type: CargoType,

    /// Class B Vendor ID
    pub equipment_vendor_id: Option<String>,

    /// Class B unite model code
    pub equipment_model: Option<u8>,

    /// Class B serial number
    pub equipment_serial_number: Option<u32>,

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

    /// ETA (20 bits)
    pub eta: Option<DateTime<Utc>>,

    /// Maximum present static draught in decimetres (1-255; 8 bits)
    pub draught10: Option<u8>,

    /// Destination (120 ASCII characters)
    pub destination: Option<String>,

    /// Class B mothership MMSI
    pub mothership_mmsi: Option<u32>,
}

// -------------------------------------------------------------------------------------------------

/// Ship type derived from combined ship and cargo type field
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShipType {
    NotAvailable = 0,             // 0
    Reserved1 = 10,               // 1x
    WingInGround = 20,            // 2x
    Fishing = 30,                 // 30
    Towing = 31,                  // 31
    TowingLong = 32,              // 32; Towing: length exceeds 200m or breadth exceeds 25m
    DredgingOrUnderwaterOps = 33, // 33
    DivingOps = 34,               // 34
    MilitaryOps = 35,             // 35
    Sailing = 36,                 // 36
    PleasureCraft = 37,           // 37
    Reserved38 = 38,              // 38
    Reserved39 = 39,              // 39
    HighSpeedCraft = 40,          // 4x
    Pilot = 50,                   // 50
    SearchAndRescue = 51,         // 51
    Tug = 52,                     // 52
    PortTender = 53,              // 53
    AntiPollutionEquipment = 54,  // 54
    LawEnforcement = 55,          // 55
    SpareLocal56 = 56,            // 56
    SpareLocal57 = 57,            // 57
    MedicalTransport = 58,        // 58
    Noncombatant = 59,            // 59; Noncombatant ship according to RR Resolution No. 18
    Passenger = 60,               // 6x
    Cargo = 70,                   // 7x
    Tanker = 80,                  // 8x
    Other = 90,                   // 9x
}

impl ShipType {
    /// Construct a new `ShipType` using the higher bits of the ship and cargo type field of NMEA.
    pub fn new(raw: u8) -> ShipType {
        match raw {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => ShipType::NotAvailable,
            10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 => ShipType::Reserved1,
            20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => ShipType::WingInGround,

            30 => ShipType::Fishing,
            31 => ShipType::Towing,
            32 => ShipType::TowingLong,
            33 => ShipType::DredgingOrUnderwaterOps,
            34 => ShipType::DivingOps,
            35 => ShipType::MilitaryOps,
            36 => ShipType::Sailing,
            37 => ShipType::PleasureCraft,
            38 => ShipType::Reserved38,
            39 => ShipType::Reserved39,

            40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 | 48 | 49 => ShipType::HighSpeedCraft,

            50 => ShipType::Pilot,
            51 => ShipType::SearchAndRescue,
            52 => ShipType::Tug,
            53 => ShipType::PortTender,
            54 => ShipType::AntiPollutionEquipment,
            55 => ShipType::LawEnforcement,
            56 => ShipType::SpareLocal56,
            57 => ShipType::SpareLocal57,
            58 => ShipType::MedicalTransport,
            59 => ShipType::Noncombatant,

            60 | 61 | 62 | 63 | 64 | 65 | 66 | 67 | 68 | 69 => ShipType::Passenger,
            70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 => ShipType::Cargo,
            80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 => ShipType::Tanker,
            90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 => ShipType::Other,
            _ => {
                warn!("Unexpected ship and cargo type: {}", raw);
                ShipType::NotAvailable
            }
        }
    }

    pub fn to_value(&self) -> u8 {
        *self as u8
    }
}

impl Default for ShipType {
    fn default() -> ShipType {
        ShipType::NotAvailable
    }
}

impl core::fmt::Display for ShipType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ShipType::NotAvailable => write!(f, "(not available)"),
            ShipType::Reserved1 => write!(f, "(reserved)"),
            ShipType::WingInGround => write!(f, "wing in ground"),
            ShipType::Fishing => write!(f, "fishing"),
            ShipType::Towing => write!(f, "towing"),
            ShipType::TowingLong => write!(f, "towing, long"),
            ShipType::DredgingOrUnderwaterOps => write!(f, "dredging or underwater ops"),
            ShipType::DivingOps => write!(f, "diving ops"),
            ShipType::MilitaryOps => write!(f, "military ops"),
            ShipType::Sailing => write!(f, "sailing"),
            ShipType::PleasureCraft => write!(f, "pleasure craft"),
            ShipType::Reserved38 => write!(f, "(reserved)"),
            ShipType::Reserved39 => write!(f, "(reserved)"),
            ShipType::HighSpeedCraft => write!(f, "high-speed craft"),
            ShipType::Pilot => write!(f, "pilot"),
            ShipType::SearchAndRescue => write!(f, "search and rescue"),
            ShipType::Tug => write!(f, "tug"),
            ShipType::PortTender => write!(f, "port tender"),
            ShipType::AntiPollutionEquipment => write!(f, "anti-pollution equipment"),
            ShipType::LawEnforcement => write!(f, "law enforcement"),
            ShipType::SpareLocal56 => write!(f, "(local)"),
            ShipType::SpareLocal57 => write!(f, "(local)"),
            ShipType::MedicalTransport => write!(f, "medical transport"),
            ShipType::Noncombatant => write!(f, "noncombatant"),
            ShipType::Passenger => write!(f, "passenger"),
            ShipType::Cargo => write!(f, "cargo"),
            ShipType::Tanker => write!(f, "tanker"),
            ShipType::Other => write!(f, "other"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Cargo type derived from combined ship and cargo type field
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CargoType {
    Undefined = 10,          // x0
    HazardousCategoryA = 11, // x1
    HazardousCategoryB = 12, // x2
    HazardousCategoryC = 13, // x3
    HazardousCategoryD = 14, // x4
    Reserved5 = 15,          // x5
    Reserved6 = 16,          // x6
    Reserved7 = 17,          // x7
    Reserved8 = 18,          // x8
    Reserved9 = 19,          // x9
}

impl CargoType {
    /// Construct a new `CargoType` using the higher bits of the ship and cargo type field of NMEA.
    pub fn new(raw: u8) -> CargoType {
        match raw {
            10 | 20 | 40 | 60 | 70 | 80 | 90 => CargoType::Undefined,
            11 | 21 | 41 | 61 | 71 | 81 | 91 => CargoType::HazardousCategoryA,
            12 | 22 | 42 | 62 | 72 | 82 | 92 => CargoType::HazardousCategoryB,
            13 | 23 | 43 | 63 | 73 | 83 | 93 => CargoType::HazardousCategoryC,
            14 | 24 | 44 | 64 | 74 | 84 | 94 => CargoType::HazardousCategoryD,
            15 | 25 | 45 | 65 | 75 | 85 | 95 => CargoType::Reserved5,
            16 | 26 | 46 | 66 | 76 | 86 | 96 => CargoType::Reserved6,
            17 | 27 | 47 | 67 | 77 | 87 | 97 => CargoType::Reserved7,
            18 | 28 | 48 | 68 | 78 | 88 | 98 => CargoType::Reserved8,
            19 | 29 | 49 | 69 | 79 | 89 | 99 => CargoType::Reserved9,
            _ => {
                warn!("Unexpected ship and cargo type: {}", raw);
                CargoType::Undefined
            }
        }
    }

    pub fn to_value(&self) -> u8 {
        *self as u8
    }
}

impl core::fmt::Display for CargoType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CargoType::Undefined => write!(f, "undefined"),
            CargoType::HazardousCategoryA => write!(f, "hazardous category A"),
            CargoType::HazardousCategoryB => write!(f, "hazardous category B"),
            CargoType::HazardousCategoryC => write!(f, "hazardous category C"),
            CargoType::HazardousCategoryD => write!(f, "hazardous category D"),
            CargoType::Reserved5 => write!(f, "(reserved)"),
            CargoType::Reserved6 => write!(f, "(reserved)"),
            CargoType::Reserved7 => write!(f, "(reserved)"),
            CargoType::Reserved8 => write!(f, "(reserved)"),
            CargoType::Reserved9 => write!(f, "(reserved)"),
        }
    }
}

impl Default for CargoType {
    fn default() -> CargoType {
        CargoType::Undefined
    }
}

// -------------------------------------------------------------------------------------------------

/// EPFD position fix types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionFixType {
    Undefined = 0,                  // 0
    GPS = 1,                        // 1
    GLONASS = 2,                    // 2
    GPSGLONASS = 3,                 // 3
    LoranC = 4,                     // 4
    Chayka = 5,                     // 5
    IntegratedNavigationSystem = 6, // 6
    Surveyed = 7,                   // 7
    Galileo = 8,                    // 8
}

impl PositionFixType {
    pub fn new(raw: u8) -> PositionFixType {
        match raw {
            0 => PositionFixType::Undefined,
            1 => PositionFixType::GPS,
            2 => PositionFixType::GLONASS,
            3 => PositionFixType::GPSGLONASS,
            4 => PositionFixType::LoranC,
            5 => PositionFixType::Chayka,
            6 => PositionFixType::IntegratedNavigationSystem,
            7 => PositionFixType::Surveyed,
            8 => PositionFixType::Galileo,
            _ => {
                warn!("Unrecognized position fix type: {}", raw);
                PositionFixType::Undefined
            }
        }
    }

    pub fn to_value(&self) -> u8 {
        *self as u8
    }
}

impl core::fmt::Display for PositionFixType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PositionFixType::Undefined => write!(f, "undefined"),
            PositionFixType::GPS => write!(f, "GPS"),
            PositionFixType::GLONASS => write!(f, "GLONASS"),
            PositionFixType::GPSGLONASS => write!(f, "GPS/GLONASS"),
            PositionFixType::LoranC => write!(f, "Loran-C"),
            PositionFixType::Chayka => write!(f, "Chayka"),
            PositionFixType::IntegratedNavigationSystem => {
                write!(f, "integrated navigation system")
            }
            PositionFixType::Surveyed => write!(f, "surveyed"),
            PositionFixType::Galileo => write!(f, "Galileo"),
        }
    }
}

impl VesselStaticData {
    /// Decode ISO 3166 country code from MID part of MMSI.
    pub fn country(&self) -> Option<&'static str> {
        match self.mmsi / 1000000 {
            // Mapping generated with mid-to-iso3166.py
            201 => Some("AL"), // Albania
            202 => Some("AD"), // Andorra
            203 => Some("AT"), // Austria
            204 => Some("PT"), // Portugal
            205 => Some("BE"), // Belgium
            206 => Some("BY"), // Belarus
            207 => Some("BG"), // Bulgaria
            208 => Some("VA"), // Vatican City State
            209 => Some("CY"), // Cyprus
            210 => Some("CY"), // Cyprus
            211 => Some("DE"), // Germany
            212 => Some("CY"), // Cyprus
            213 => Some("GE"), // Georgia
            214 => Some("MD"), // Moldova
            215 => Some("MT"), // Malta
            216 => Some("AM"), // Armenia
            218 => Some("DE"), // Germany
            219 => Some("DK"), // Denmark
            220 => Some("DK"), // Denmark
            224 => Some("ES"), // Spain
            225 => Some("ES"), // Spain
            226 => Some("FR"), // France
            227 => Some("FR"), // France
            228 => Some("FR"), // France
            229 => Some("MT"), // Malta
            230 => Some("FI"), // Finland
            231 => Some("FO"), // Faroe Islands
            232 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
            233 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
            234 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
            235 => Some("GB"), // United Kingdom of Great Britain and Northern Ireland
            236 => Some("GI"), // Gibraltar
            237 => Some("GR"), // Greece
            238 => Some("HR"), // Croatia
            239 => Some("GR"), // Greece
            240 => Some("GR"), // Greece
            241 => Some("GR"), // Greece
            242 => Some("MA"), // Morocco
            243 => Some("HU"), // Hungary
            244 => Some("NL"), // Netherlands
            245 => Some("NL"), // Netherlands
            246 => Some("NL"), // Netherlands
            247 => Some("IT"), // Italy
            248 => Some("MT"), // Malta
            249 => Some("MT"), // Malta
            250 => Some("IE"), // Ireland
            251 => Some("IS"), // Iceland
            252 => Some("LI"), // Liechtenstein
            253 => Some("LU"), // Luxembourg
            254 => Some("MC"), // Monaco
            255 => Some("PT"), // Portugal
            256 => Some("MT"), // Malta
            257 => Some("NO"), // Norway
            258 => Some("NO"), // Norway
            259 => Some("NO"), // Norway
            261 => Some("PL"), // Poland
            262 => Some("ME"), // Montenegro
            263 => Some("PT"), // Portugal
            264 => Some("RO"), // Romania
            265 => Some("SE"), // Sweden
            266 => Some("SE"), // Sweden
            267 => Some("SK"), // Slovakia
            268 => Some("SM"), // San Marino
            269 => Some("CH"), // Switzerland
            270 => Some("CZ"), // Czechia
            271 => Some("TR"), // Turkey
            272 => Some("UA"), // Ukraine
            273 => Some("RU"), // Russian Federation
            274 => Some("MK"), // Republic of North Macedonia
            275 => Some("LV"), // Latvia
            276 => Some("EE"), // Estonia
            277 => Some("LT"), // Lithuania
            278 => Some("SI"), // Slovenia
            279 => Some("RS"), // Serbia
            301 => Some("AI"), // Anguilla
            303 => Some("US"), // United States of America
            304 => Some("AG"), // Antigua and Barbuda
            305 => Some("AG"), // Antigua and Barbuda
            306 => Some("BQ"), // Bonaire, Sint Eustatius and Saba
            //            306 => Some("CW"), // Curaçao
            //            306 => Some("SX"), // Sint Maarten
            307 => Some("AW"), // Aruba
            308 => Some("BS"), // Bahamas
            309 => Some("BS"), // Bahamas
            310 => Some("BM"), // Bermuda
            311 => Some("BS"), // Bahamas
            312 => Some("BZ"), // Belize
            314 => Some("BB"), // Barbados
            316 => Some("CA"), // Canada
            319 => Some("KY"), // Cayman Islands
            321 => Some("CR"), // Costa Rica
            323 => Some("CU"), // Cuba
            325 => Some("DM"), // Dominica
            327 => Some("DO"), // Dominican Republic
            329 => Some("GP"), // Guadeloupe
            330 => Some("GD"), // Grenada
            331 => Some("GL"), // Greenland
            332 => Some("GT"), // Guatemala
            334 => Some("HN"), // Honduras
            336 => Some("HT"), // Haiti
            338 => Some("US"), // United States of America
            339 => Some("JM"), // Jamaica
            341 => Some("KN"), // Saint Kitts and Nevis
            343 => Some("LC"), // Saint Lucia
            345 => Some("MX"), // Mexico
            347 => Some("MQ"), // Martinique
            348 => Some("MS"), // Montserrat
            350 => Some("NI"), // Nicaragua
            351 => Some("PA"), // Panama
            352 => Some("PA"), // Panama
            353 => Some("PA"), // Panama
            354 => Some("PA"), // Panama
            355 => Some("PA"), // Panama
            356 => Some("PA"), // Panama
            357 => Some("PA"), // Panama
            358 => Some("PR"), // Puerto Rico
            359 => Some("SV"), // El Salvador
            361 => Some("PM"), // Saint Pierre and Miquelon
            362 => Some("TT"), // Trinidad and Tobago
            364 => Some("TC"), // Turks and Caicos Islands
            366 => Some("US"), // United States of America
            367 => Some("US"), // United States of America
            368 => Some("US"), // United States of America
            369 => Some("US"), // United States of America
            370 => Some("PA"), // Panama
            371 => Some("PA"), // Panama
            372 => Some("PA"), // Panama
            373 => Some("PA"), // Panama
            374 => Some("PA"), // Panama
            375 => Some("VC"), // Saint Vincent and the Grenadines
            376 => Some("VC"), // Saint Vincent and the Grenadines
            377 => Some("VC"), // Saint Vincent and the Grenadines
            378 => Some("VG"), // British Virgin Islands
            379 => Some("VI"), // United States Virgin Islands
            401 => Some("AF"), // Afghanistan
            403 => Some("SA"), // Saudi Arabia
            405 => Some("BD"), // Bangladesh
            408 => Some("BH"), // Bahrain
            410 => Some("BT"), // Bhutan
            412 => Some("CN"), // China
            413 => Some("CN"), // China
            414 => Some("CN"), // China
            416 => Some("TW"), // Taiwan
            417 => Some("LK"), // Sri Lanka
            419 => Some("IN"), // India
            422 => Some("IR"), // Iran
            423 => Some("AZ"), // Azerbaijan
            425 => Some("IQ"), // Iraq
            428 => Some("IL"), // Israel
            431 => Some("JP"), // Japan
            432 => Some("JP"), // Japan
            434 => Some("TM"), // Turkmenistan
            436 => Some("KZ"), // Kazakhstan
            437 => Some("UZ"), // Uzbekistan
            438 => Some("JO"), // Jordan
            440 => Some("KR"), // Korea
            441 => Some("KR"), // Korea
            443 => Some("PS"), // Palestine, State of
            445 => Some("KR"), // Korea
            447 => Some("KW"), // Kuwait
            450 => Some("LB"), // Lebanon
            451 => Some("KG"), // Kyrgyzstan
            453 => Some("MO"), // Macao
            455 => Some("MV"), // Maldives
            457 => Some("MN"), // Mongolia
            459 => Some("NP"), // Nepal
            461 => Some("OM"), // Oman
            463 => Some("PK"), // Pakistan
            466 => Some("QA"), // Qatar
            468 => Some("SY"), // Syrian Arab Republic
            470 => Some("AE"), // United Arab Emirates
            471 => Some("AE"), // United Arab Emirates
            472 => Some("TJ"), // Tajikistan
            473 => Some("YE"), // Yemen
            475 => Some("YE"), // Yemen
            477 => Some("HK"), // Hong Kong
            478 => Some("BA"), // Bosnia and Herzegovina
            501 => Some("TF"), // French Southern Territories
            503 => Some("AU"), // Australia
            506 => Some("MM"), // Myanmar
            508 => Some("BN"), // Brunei Darussalam
            510 => Some("FM"), // Micronesia
            511 => Some("PW"), // Palau
            512 => Some("NZ"), // New Zealand
            514 => Some("KH"), // Cambodia
            515 => Some("KH"), // Cambodia
            516 => Some("CX"), // Christmas Island
            518 => Some("CK"), // Cook Islands
            520 => Some("FJ"), // Fiji
            523 => Some("CC"), // Cocos Islands
            525 => Some("ID"), // Indonesia
            529 => Some("KI"), // Kiribati
            531 => Some("LA"), // Lao People's Democratic Republic
            533 => Some("MY"), // Malaysia
            536 => Some("MP"), // Northern Mariana Islands
            538 => Some("MH"), // Marshall Islands
            540 => Some("NC"), // New Caledonia
            542 => Some("NU"), // Niue
            544 => Some("NR"), // Nauru
            546 => Some("PF"), // French Polynesia
            548 => Some("PH"), // Philippines
            550 => Some("TL"), // Timor-Leste
            553 => Some("PG"), // Papua New Guinea
            555 => Some("PN"), // Pitcairn
            557 => Some("SB"), // Solomon Islands
            559 => Some("AS"), // American Samoa
            561 => Some("WS"), // Samoa
            563 => Some("SG"), // Singapore
            564 => Some("SG"), // Singapore
            565 => Some("SG"), // Singapore
            566 => Some("SG"), // Singapore
            567 => Some("TH"), // Thailand
            570 => Some("TO"), // Tonga
            572 => Some("TV"), // Tuvalu
            574 => Some("VN"), // Viet Nam
            576 => Some("VU"), // Vanuatu
            577 => Some("VU"), // Vanuatu
            578 => Some("WF"), // Wallis and Futuna
            601 => Some("ZA"), // South Africa
            603 => Some("AO"), // Angola
            605 => Some("DZ"), // Algeria
            607 => Some("TF"), // French Southern Territories
            608 => Some("SH"), // Saint Helena, Ascension and Tristan da Cunha
            609 => Some("BI"), // Burundi
            610 => Some("BJ"), // Benin
            611 => Some("BW"), // Botswana
            612 => Some("CF"), // Central African Republic
            613 => Some("CM"), // Cameroon
            615 => Some("CG"), // Congo
            616 => Some("KM"), // Comoros
            617 => Some("CV"), // Cabo Verde
            618 => Some("TF"), // French Southern Territories
            619 => Some("CI"), // Côte d'Ivoire
            620 => Some("KM"), // Comoros
            621 => Some("DJ"), // Djibouti
            622 => Some("EG"), // Egypt
            624 => Some("ET"), // Ethiopia
            625 => Some("ER"), // Eritrea
            626 => Some("GA"), // Gabon
            627 => Some("GH"), // Ghana
            629 => Some("GM"), // Gambia
            630 => Some("GW"), // Guinea-Bissau
            631 => Some("GQ"), // Equatorial Guinea
            632 => Some("GN"), // Guinea
            633 => Some("BF"), // Burkina Faso
            634 => Some("KE"), // Kenya
            635 => Some("TF"), // French Southern Territories
            636 => Some("LR"), // Liberia
            637 => Some("LR"), // Liberia
            638 => Some("SS"), // South Sudan
            642 => Some("LY"), // Libya
            644 => Some("LS"), // Lesotho
            645 => Some("MU"), // Mauritius
            647 => Some("MG"), // Madagascar
            649 => Some("ML"), // Mali
            650 => Some("MZ"), // Mozambique
            654 => Some("MR"), // Mauritania
            655 => Some("MW"), // Malawi
            656 => Some("NE"), // Niger
            657 => Some("NG"), // Nigeria
            659 => Some("NA"), // Namibia
            660 => Some("TF"), // French Southern Territories
            661 => Some("RW"), // Rwanda
            662 => Some("SD"), // Sudan
            663 => Some("SN"), // Senegal
            664 => Some("SC"), // Seychelles
            665 => Some("SH"), // Saint Helena, Ascension and Tristan da Cunha
            666 => Some("SO"), // Somalia
            667 => Some("SL"), // Sierra Leone
            668 => Some("ST"), // Sao Tome and Principe
            669 => Some("SZ"), // Eswatini
            670 => Some("TD"), // Chad
            671 => Some("TG"), // Togo
            672 => Some("TN"), // Tunisia
            674 => Some("TZ"), // Tanzania, United Republic of
            675 => Some("UG"), // Uganda
            676 => Some("CG"), // Congo
            677 => Some("TZ"), // Tanzania, United Republic of
            678 => Some("ZM"), // Zambia
            679 => Some("ZW"), // Zimbabwe
            701 => Some("AR"), // Argentina
            710 => Some("BR"), // Brazil
            720 => Some("BO"), // Bolivia
            725 => Some("CL"), // Chile
            730 => Some("CO"), // Colombia
            735 => Some("EC"), // Ecuador
            740 => Some("FK"), // Falkland Islands [Malvinas]
            745 => Some("GF"), // French Guiana
            750 => Some("GY"), // Guyana
            755 => Some("PY"), // Paraguay
            760 => Some("PE"), // Peru
            765 => Some("SR"), // Suriname
            770 => Some("UY"), // Uruguay
            775 => Some("VE"), // Venezuela
            _ => None,
        }
    }
}
