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

#[doc(hidden)]
pub mod gga;
#[doc(hidden)]
pub mod gsa;
#[doc(hidden)]
pub mod gsv;
#[doc(hidden)]
pub mod rmc;
#[doc(hidden)]
pub mod vtg;
#[doc(hidden)]
pub mod gll;

use super::*;

// -------------------------------------------------------------------------------------------------
/// GGA - time, position, and fix related data
#[derive(Clone, Debug, PartialEq)]
pub struct GgaData {
    /// Navigation system
    pub system: NavigationSystem,

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
    Invalid, // 0
    GpsFix, // 1
    DGpsFix, // 2
    PpsFix, // 3
    RealTimeKinematic, // 4
    RealTimeKinematicFloat, // 5
    DeadReckoning, // 6
    ManualInputMode, // 7
    SimulationMode, // 8
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

impl std::fmt::Display for GgaQualityIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GgaQualityIndicator::Invalid => { write!(f, "invalid") }
            GgaQualityIndicator::GpsFix => { write!(f, "GPS fix") }
            GgaQualityIndicator::DGpsFix => { write!(f, "DGPS fix") }
            GgaQualityIndicator::PpsFix => { write!(f, "PPS fix") }
            GgaQualityIndicator::RealTimeKinematic => { write!(f, "Real-Time Kinematic") }
            GgaQualityIndicator::RealTimeKinematicFloat => { write!(f, "Real-Time Kinematic (floating point)") }
            GgaQualityIndicator::DeadReckoning => { write!(f, "dead reckoning") }
            GgaQualityIndicator::ManualInputMode => { write!(f, "manual input mode") }
            GgaQualityIndicator::SimulationMode => { write!(f, "simulation mode") }
        }
        
    }
}

// -------------------------------------------------------------------------------------------------

/// Navigation system, deptected from NMEA GNSS sentence prefix (e.g. $BDGGA)
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
/// RMC - position, velocity, and time (Recommended Minimum sentence C)
#[derive(Clone, Debug, PartialEq)]
pub struct RmcData {
    /// Navigation system
    pub system: NavigationSystem,

    /// Fix datetime based on HHMMSS and DDMMYY
    pub timestamp: Option<DateTime<Utc>>,
    
    /// Status: true = active, false = void.
    pub status_active: Option<bool>,
    
    /// Latitude in degrees    
    pub latitude: Option<f64>,

    /// Longitude in degrees    
    pub longitude: Option<f64>,
    
    /// Speed over ground in knots
    pub speed_knots: Option<f64>,
    
    /// Track angle in degrees (True)
    pub bearing: Option<f64>,
    
    /// Magnetic variation in degrees
    pub variation: Option<f64>
}

impl LatLon for RmcData {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

// -------------------------------------------------------------------------------------------------
/// GSA - GNSS dilution of position (DOP) and active satellites
#[derive(Clone, Debug, PartialEq)]
pub struct GsaData {
    /// Navigation system
    pub system: NavigationSystem,

    /// Mode 1: true = automatic, false = manual
    pub mode1_automatic: Option<bool>,
    
    /// Mode 2, fix type: 
    pub mode2_3d: Option<GsaFixMode>,
    
    /// PRN numbers used (space for 12)
    pub prn_numbers: Vec<u8>,
    
    /// Position (3D) dilution of precision
    pub pdop: Option<f64>,
    
    /// Horizontal dilution of precision
    pub hdop: Option<f64>,
    
    /// Vertical dilution of precision
    pub vdop: Option<f64>,
}

/// GSA position fix type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GsaFixMode {
    /// No fix.
    NotAvailable,
    
    /// 2D fix.
    Fix2D,
    
    /// 3d fix.
    Fix3D,
}

impl std::fmt::Display for GsaFixMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GsaFixMode::NotAvailable => { write!(f, "no available") }
            GsaFixMode::Fix2D => { write!(f, "2D fix") }
            GsaFixMode::Fix3D => { write!(f, "3D fix") }
        }
        
    }
}

// -------------------------------------------------------------------------------------------------
/// GSV - satellite information
#[derive(Clone, Debug, PartialEq)]
pub struct GsvData {
    /// Navigation system
    pub system: NavigationSystem,

    /// Satellite PRN number
    pub prn_number: u8,
    
    /// Elevation in degrees (max 90°)
    pub elevation: Option<u8>,
    
    /// Azimuth in degrees from True north (0°-359°)
    pub azimuth: Option<u16>,
    
    /// SNR, 0-99 dB, None when not tracking
    pub snr: Option<u8>,
}

// -------------------------------------------------------------------------------------------------

/// VTG - track made good and speed over ground
#[derive(Clone, Debug, PartialEq)]
pub struct VtgData {
    /// Navigation system
    pub system: NavigationSystem,

    /// Course over ground (CoG), degrees True
    pub cog_true: Option<f64>,
    
    /// Course over ground (CoG), degrees Magnetic
    pub cog_magnetic: Option<f64>,
    
    /// Speed over ground (SoG), knots
    pub sog_knots: Option<f64>,
    
    /// Speed over ground (SoG), km/h
    pub sog_kph: Option<f64>,
    
    /// FAA mode indicator
    pub faa_mode: Option<FaaMode>,
}

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

// -------------------------------------------------------------------------------------------------

/// GLL - geographic Position - Latitude/Longitude
#[derive(Clone, Debug, PartialEq)]
pub struct GllData {
    /// Navigation system
    pub system: NavigationSystem,

    /// Latitude in degrees.
    pub latitude: Option<f64>,

    /// Longitude in degrees.
    pub longitude: Option<f64>,

    /// UTC of position fix
    pub timestamp: Option<DateTime<Utc>>,

    /// True = data valid, false = data invalid.
    pub data_valid: Option<bool>,
    
    /// FAA mode indicator (NMEA 2.3 and later).
    pub faa_mode: Option<FaaMode>,
}

impl LatLon for GllData {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

