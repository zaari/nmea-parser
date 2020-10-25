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

//! GNSS-related data structures

pub(crate) mod gga;
pub(crate) mod gsa;
pub(crate) mod gsv;
pub(crate) mod rmc;
pub(crate) mod vtg;
pub(crate) mod gll;

use super::*;
pub use rmc::RmcData;
pub use gga::{GgaData, GgaQualityIndicator};
pub use gll::GllData;
pub use gsa::{GsaData,GsaFixMode};
pub use gsv::GsvData;
pub use vtg::VtgData;

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
    Beidou,  // BDxxx
    
    // Indian NavIC
    Navic,  // GIxxx
    
    // Japanese Qzss
    Qzss,  // QZxxx
    
    // Some other
    Other,
}

impl std::fmt::Display for NavigationSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NavigationSystem::Combination => { write!(f, "combination") }
            NavigationSystem::Gps => { write!(f, "GPS") }
            NavigationSystem::Glonass => { write!(f, "GLONASS") }
            NavigationSystem::Galileo => { write!(f, "Galileo") }
            NavigationSystem::Beidou => { write!(f, "BeiDou") }
            NavigationSystem::Navic => { write!(f, "Navic") }
            NavigationSystem::Qzss => { write!(f, "QZSS") }
            NavigationSystem::Other => { write!(f, "other") }
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
            _ => { Err(format!("Unrecognized FAA information value: {}", val)) }
        }
    }
}

impl std::fmt::Display for FaaMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FaaMode::Autonomous => { write!(f, "A") }
            FaaMode::Differential => { write!(f, "D") }
            FaaMode::Estimated => { write!(f, "E") }
            FaaMode::NotValid => { write!(f, "N") }
            _ => { write!(f, "?") }
        }
        
    }
}

