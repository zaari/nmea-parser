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

/// Type 23: Group Assignment Command
#[derive(Default, Clone, Debug, PartialEq)]
pub struct GroupAssignmentCommand {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// Northeast latitude to 0.1 minutes.
    pub ne_lat: Option<f64>,

    /// Northeast longitude to 0.1 minutes.
    pub ne_lon: Option<f64>,

    /// Southwest latitude to 0.1 minutes.
    pub sw_lat: Option<f64>,

    /// Southwest longitude to 0.1 minutes.
    pub sw_lon: Option<f64>,

    /// AIS station type.
    pub station_type: StationType,

    /// Ship type
    pub ship_type: ShipType,

    /// Cargo type
    pub cargo_type: CargoType,

    /// TxRx mode:
    /// 0 = TxA/TxB, RxA/RxB (default)
    /// 1 = TxA, RxA/RxB
    /// 2 = TxB, RxA/RxB
    /// 3 = Reserved for future use
    pub txrx: u8,

    /// Report interval.
    pub interval: StationInterval,

    /// Quiet time specifies how many minutes the affected stations are to remain silent.
    /// None = none
    /// 1-15 = quiet time in minutes
    pub quiet: Option<u8>,
}

/// Station Type (for message type 23).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StationType {
    /// All types of mobiles (default)
    AllTypes,

    /// Reserved for future use
    Reserved1,

    /// All types of Class B mobile stations
    AllTypesOfClassBMobile,

    /// SAR airborne mobile station
    SarAirborneMobile,

    /// Aid to Navigation station
    AidToNavigation,

    /// Class B shopborne mobile station (IEC62287 only)
    ClassBShipBorneMobile,

    /// Regional use and inland waterways
    Regional6,

    /// Regional use and inland waterways
    Regional7,

    /// Regional use and inland waterways
    Regional8,

    /// Regional use and inland waterways
    Regional9,

    /// Reserved for future use
    Reserved10,

    /// Reserved for future use
    Reserved11,

    /// Reserved for future use
    Reserved12,

    /// Reserved for future use
    Reserved13,

    /// Reserved for future use
    Reserved14,

    /// Reserved for future use
    Reserved15,
}

impl Default for StationType {
    fn default() -> Self {
        StationType::AllTypes
    }
}

impl StationType {
    fn new(val: u8) -> Result<StationType, String> {
        match val {
            0 => Ok(StationType::AllTypes),
            1 => Ok(StationType::Reserved1),
            2 => Ok(StationType::AllTypesOfClassBMobile),
            3 => Ok(StationType::SarAirborneMobile),
            4 => Ok(StationType::AidToNavigation),
            5 => Ok(StationType::ClassBShipBorneMobile),
            6 => Ok(StationType::Regional6),
            7 => Ok(StationType::Regional7),
            8 => Ok(StationType::Regional8),
            9 => Ok(StationType::Regional9),
            10 => Ok(StationType::Reserved10),
            11 => Ok(StationType::Reserved11),
            12 => Ok(StationType::Reserved12),
            13 => Ok(StationType::Reserved13),
            14 => Ok(StationType::Reserved14),
            15 => Ok(StationType::Reserved15),
            _ => Err(format!("Station type value out of range: {}", val)),
        }
    }
}

/// Station interval (for message type 23)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StationInterval {
    /// As given by the autonomous mode
    Autonomous,

    /// 10 minutes
    Time10min,

    /// 6 minutes
    Time6min,

    /// 3 minutes
    Time3min,

    /// 1 minute
    Time1min,

    /// 30 seconds
    Time30sec,

    /// 15 seconds
    Time15sec,

    /// 10 seconds
    Time10sec,

    /// 5 seconds
    Time5sec,

    /// Next shorter reporting interval
    NextShorterReportingInverval,

    /// Next longer reporting interval
    NextLongerReportingInverval,

    /// Reserved for future use
    Reserved11,

    /// Reserved for future use
    Reserved12,

    /// Reserved for future use
    Reserved13,

    /// Reserved for future use
    Reserved14,

    /// Reserved for future use
    Reserved15,
}

impl StationInterval {
    fn new(val: u8) -> Result<StationInterval, String> {
        match val {
            0 => Ok(StationInterval::Autonomous),
            1 => Ok(StationInterval::Time10min),
            2 => Ok(StationInterval::Time6min),
            3 => Ok(StationInterval::Time3min),
            4 => Ok(StationInterval::Time1min),
            5 => Ok(StationInterval::Time30sec),
            6 => Ok(StationInterval::Time15sec),
            7 => Ok(StationInterval::Time10sec),
            8 => Ok(StationInterval::Time5sec),
            9 => Ok(StationInterval::NextShorterReportingInverval),
            10 => Ok(StationInterval::NextLongerReportingInverval),
            11 => Ok(StationInterval::Reserved11),
            12 => Ok(StationInterval::Reserved12),
            13 => Ok(StationInterval::Reserved13),
            14 => Ok(StationInterval::Reserved14),
            15 => Ok(StationInterval::Reserved15),
            _ => Err(format!("Station interval value out of range: {}", val)),
        }
    }
}

impl Default for StationInterval {
    fn default() -> Self {
        StationInterval::Autonomous
    }
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 23: Group Assignment Command
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::GroupAssignmentCommand(
        GroupAssignmentCommand {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            ne_lat: { Some(pick_i64(&bv, 58, 17) as f64 / 600.0) },
            ne_lon: { Some(pick_i64(&bv, 40, 18) as f64 / 600.0) },
            sw_lat: { Some(pick_i64(&bv, 93, 17) as f64 / 600.0) },
            sw_lon: { Some(pick_i64(&bv, 75, 18) as f64 / 600.0) },
            station_type: StationType::new(pick_u64(&bv, 110, 4) as u8)?,
            ship_type: ShipType::new(pick_u64(&bv, 114, 8) as u8),
            cargo_type: CargoType::new(pick_u64(&bv, 114, 8) as u8),
            txrx: {
                let val = pick_u64(&bv, 144, 2) as u8;
                if val < 4 {
                    val
                } else {
                    return Err(format!("Tx/Tr mode field out of range: {}", val).into());
                }
            },
            interval: StationInterval::new(pick_u64(&bv, 146, 4) as u8)?,
            quiet: {
                let val = pick_u64(&bv, 144, 4) as u8;
                match val {
                    0 => None,
                    1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 => Some(val),
                    _ => {
                        unreachable!("This should never be reached as all four bit cases are covered (value: {})", val);
                    }
                }
            },
        },
    ))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type23() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,B,G02:Kn01R`sn@291nj600000900,2*12") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::GroupAssignmentCommand(gac) => {
                        assert_eq!(gac.mmsi, 2268120);
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
