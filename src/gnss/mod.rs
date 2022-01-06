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

//! GNSS data structures

pub(crate) mod gga;
pub(crate) mod gll;
pub(crate) mod gns;
pub(crate) mod gsa;
pub(crate) mod gsv;
pub(crate) mod rmc;
pub(crate) mod vtg;
pub(crate) mod alm;
pub(crate) mod dtm;
pub(crate) mod mss;
pub(crate) mod stn;
pub(crate) mod vbw;
pub(crate) mod zda;
pub(crate) mod dpt;
pub(crate) mod dbs;
pub(crate) mod mtw;
pub(crate) mod vhw;
pub(crate) mod hdt;
pub(crate) mod mwv;

use super::*;
pub use gga::{GgaData, GgaQualityIndicator};
pub use gll::GllData;
pub use gns::GnsData;
pub use gsa::{GsaData, GsaFixMode};
pub use gsv::GsvData;
pub use rmc::RmcData;
pub use vtg::VtgData;
pub use alm::AlmData;
pub use dtm::DtmData;
pub use mss::MssData;
pub use stn::StnData;
pub use vbw::VbwData;
pub use zda::ZdaData;
pub use dpt::DptData;
pub use dbs::DbsData;
pub use mtw::MtwData;
pub use vhw::VhwData;
pub use hdt::HdtData;
pub use mwv::MwvData;

// -------------------------------------------------------------------------------------------------

/// Navigation system, identified with NMEA GNSS sentence prefix (e.g. $BDGGA)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavigationSystem {
    /// Combination of several satellite systems
    Combination, // GNxxx

    /// American GPS
    Gps, // GPxxx

    /// Russian GLONASS
    Glonass, // GLxxx

    /// European Galileo
    Galileo, // GAxxx

    // Chinese BeiDou
    Beidou, // BDxxx

    // Indian NavIC
    Navic, // GIxxx

    // Japanese Qzss
    Qzss, // QZxxx

    /// Proprietary manufacturer specific message
    Proprietary, // PMMM, P usually followed by a three character manufacturer code

    // Some other
    Other,
}

impl core::fmt::Display for NavigationSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NavigationSystem::Combination => write!(f, "combination"),
            NavigationSystem::Gps => write!(f, "GPS"),
            NavigationSystem::Glonass => write!(f, "GLONASS"),
            NavigationSystem::Galileo => write!(f, "Galileo"),
            NavigationSystem::Beidou => write!(f, "BeiDou"),
            NavigationSystem::Navic => write!(f, "Navic"),
            NavigationSystem::Qzss => write!(f, "QZSS"),
            NavigationSystem::Proprietary => write!(f, "proprietary"),
            NavigationSystem::Other => write!(f, "other"),
        }
    }
}

impl core::str::FromStr for NavigationSystem {
    type Err = ParseError;

    fn from_str(talker_id: &str) -> Result<Self, Self::Err> {
        if talker_id.len() < 1 {
            return Err(ParseError::InvalidSentence(
                "Invalid talker identifier".to_string(),
            ));
        }
        if &talker_id[0..1] == "P" {
            Ok(Self::Proprietary)
        } else {
            if talker_id.len() < 2 {
                return Err(ParseError::InvalidSentence(
                    "Invalid talker identifier".to_string(),
                ));
            }
            match &talker_id[0..2] {
                "GN" => Ok(Self::Combination),
                "GP" => Ok(Self::Gps),
                "GL" => Ok(Self::Glonass),
                "GA" => Ok(Self::Galileo),
                "BD" => Ok(Self::Beidou),
                "GI" => Ok(Self::Navic),
                "QZ" => Ok(Self::Qzss),
                _ => Ok(Self::Other),
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------
/// VTG/GLL FAA mode (NMEA 2.3 standard has this information)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FaaMode {
    /// Autonomous mode (automatic 2D/3D)
    Autonomous,

    /// Differential GPS mode (DGPS).
    Differential,

    /// Estimated (dead-reckoning) data.
    Estimated,

    /// No valid data available.
    NotValid,

    /// Simulated data.
    Simulator,
}

impl FaaMode {
    pub fn new(val: &str) -> Result<FaaMode, String> {
        match val {
            "A" => Ok(FaaMode::Autonomous),
            "D" => Ok(FaaMode::Differential),
            "E" => Ok(FaaMode::Estimated),
            "N" => Ok(FaaMode::NotValid),
            _ => Err(format!("Unrecognized FAA information value: {}", val)),
        }
    }
}

impl core::fmt::Display for FaaMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FaaMode::Autonomous => write!(f, "A"),
            FaaMode::Differential => write!(f, "D"),
            FaaMode::Estimated => write!(f, "E"),
            FaaMode::NotValid => write!(f, "N"),
            _ => write!(f, "?"),
        }
    }
}
