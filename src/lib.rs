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

//! # NMEA Parser: NMEA parser for Rust
//!
//! This crate aims to cover all AIS sentences and the most important GNSS sentences used with
//! NMEA 0183 standard. The parser supports AIS class A and B types. It also identifies GPS,
//! GLONASS, Galileo, BeiDou, NavIC and QZSS satellite systems.

#![allow(dead_code)]

#[macro_use]
extern crate log;

extern crate num_traits;

use bitvec::prelude::*;
pub use chrono;
use chrono::prelude::*;
use chrono::DateTime;
use std::collections::HashMap;

pub mod ais;
mod error;
pub mod gnss;
mod util;

pub use error::ParseError;
use util::*;

// -------------------------------------------------------------------------------------------------

/// Result from function `NmeaParser::parse_sentence()`. If the given sentence represents only a
/// partial message `ParsedMessage::Incomplete` is returned.
#[derive(Clone, Debug, PartialEq)]
pub enum ParsedMessage {
    /// The given sentence is only part of multi-sentence message and we need more data to
    /// create the actual result. State is stored in `NmeaParser` object.
    Incomplete,

    /// AIS VDM/VDO t1, t2, t3, t18 and t27
    VesselDynamicData(ais::VesselDynamicData),

    /// AIS VDM/VDO t5 and t24
    VesselStaticData(ais::VesselStaticData),

    /// AIS VDM/VDO type 4
    BaseStationReport(ais::BaseStationReport),

    /// AIS VDM/VDO type 6
    BinaryAddressedMessage(ais::BinaryAddressedMessage),
    //
    //    /// AIS VDM/VDO type 7
    //    BinaryAcknowledge(ais::BinaryAcknowledge),
    //
    //    /// AIS VDM/VDO type 8
    //    BinaryBroadcastMessage(ais::BinaryBroadcastMessage),

    // AIS VDM/VDO type 9
    StandardSarAircraftPositionReport(ais::StandardSarAircraftPositionReport),

    // AIS VDM/VDO type 10
    UtcDateInquiry(ais::UtcDateInquiry),

    // AIS VDM/VDO type 11
    UtcDateResponse(ais::BaseStationReport),

    // AIS VDM/VDO type 12
    AddressedSafetyRelatedMessage(ais::AddressedSafetyRelatedMessage),

    // AIS VDM/VDO type 13
    SafetyRelatedAcknowledgement(ais::SafetyRelatedAcknowledgement),

    // AIS VDM/VDO type 14
    SafetyRelatedBroadcastMessage(ais::SafetyRelatedBroadcastMessage),

    // AIS VDM/VRO type 15
    Interrogation(ais::Interrogation),

    // AIS VDM/VRO type 16
    AssignmentModeCommand(ais::AssignmentModeCommand),

    // AIS VDM/VRO type 17
    DgnssBroadcastBinaryMessage(ais::DgnssBroadcastBinaryMessage),

    // AIS VDM/VDO type 21
    AidToNavigationReport(ais::AidToNavigationReport),

    /// GGA
    Gga(gnss::GgaData),

    /// RMC
    Rmc(gnss::RmcData),

    /// GSA
    Gsa(gnss::GsaData),

    /// GSV
    Gsv(Vec<gnss::GsvData>),

    /// VTG
    Vtg(gnss::VtgData),

    /// GLL
    Gll(gnss::GllData),

    /// ALM
    Alm(gnss::AlmData),

    /// DTM
    Dtm(gnss::DtmData),

    /// MSS
    Mss(gnss::MssData),

    /// STN
    Stn(gnss::StnData),

    /// VBW
    Vbw(gnss::VbwData),
}

// -------------------------------------------------------------------------------------------------

/// Read-only access to geographical position in the implementing type.
pub trait LatLon {
    /// Return the latitude of the position contained by the object. If the position is not
    /// available return `None`.
    fn latitude(&self) -> Option<f64>;

    /// Return the longitude of the position contained by the object. If the position is not
    /// available return `None`.
    fn longitude(&self) -> Option<f64>;
}

// -------------------------------------------------------------------------------------------------

/// NMEA sentence parser which keeps multi-sentence state between `parse_sentence` calls.
/// The parser tries to be as permissible as possible about the field formats because some NMEA
/// encoders don't follow the standards strictly.
#[derive(Clone)]
pub struct NmeaParser {
    saved_fragments: HashMap<String, String>,
    saved_vsds: HashMap<u32, ais::VesselStaticData>,
}

impl NmeaParser {
    /// Construct an empty parser which is ready to receive sentences.
    pub fn new() -> NmeaParser {
        NmeaParser {
            saved_fragments: HashMap::new(),
            saved_vsds: HashMap::new(),
        }
    }

    /// Clear internal state of the parser. Multi-sentence state is lost when this function
    /// is called.
    pub fn reset(&mut self) {
        self.saved_fragments.clear();
        self.saved_vsds.clear();
    }

    /// Push string-to-string mapping to store.
    fn push_string(&mut self, key: String, value: String) {
        self.saved_fragments.insert(key, value);
    }

    /// Pull string-to-string mapping by key from store.
    fn pull_string(&mut self, key: String) -> Option<String> {
        self.saved_fragments.remove(&key)
    }

    /// Tests whether the given string-to-string mapping exists in the store.
    fn contains_key(&mut self, key: String) -> bool {
        self.saved_fragments.contains_key(&key)
    }

    /// Return number of string-to-string mappings stored.
    fn strings_count(&self) -> usize {
        self.saved_fragments.len()
    }

    /// Push MMSI-to-VesselStaticData mapping to store.
    fn push_vsd(&mut self, mmsi: u32, vsd: ais::VesselStaticData) {
        self.saved_vsds.insert(mmsi, vsd);
    }

    /// Pull MMSI-to-VesselStaticData mapping from store.
    fn pull_vsd(&mut self, mmsi: u32) -> Option<ais::VesselStaticData> {
        self.saved_vsds.remove(&mmsi)
    }

    /// Return number of MMSI-to-VesselStaticData mappings in store.    
    fn vsds_count(&self) -> usize {
        self.saved_vsds.len()
    }

    /// Parse NMEA sentence into `ParsedMessage` enum. If the given sentence is part of
    /// a multipart message the related state is saved into the parser and
    /// `ParsedMessage::Incomplete` is returned. The actual result is returned when all the parts
    /// have been sent to the parser.
    pub fn parse_sentence(&mut self, sentence: &str) -> Result<ParsedMessage, ParseError> {
        // Calculace NMEA checksum and compare it to the given one. Also, remove the checksum part
        // from the sentence to simplify next processing steps.
        let mut checksum = 0;
        let (sentence, checksum_hex_given) = {
            if let Some(pos) = sentence.rfind('*') {
                if pos + 3 <= sentence.len() {
                    (
                        sentence[0..pos].to_string(),
                        sentence[(pos + 1)..(pos + 3)].to_string(),
                    )
                } else {
                    debug!("Invalid checksum found for sentence: {}", sentence);
                    (sentence[0..pos].to_string(), "".to_string())
                }
            } else {
                debug!("No checksum found for sentence: {}", sentence);
                (sentence.to_string(), "".to_string())
            }
        };
        for c in sentence.as_str().chars().skip(1) {
            checksum = checksum ^ (c as u8);
        }
        let checksum_hex_calculated = format!("{:02X?}", checksum);
        if checksum_hex_calculated != checksum_hex_given && checksum_hex_given != "" {
            return Err(ParseError::CorruptedSentence(format!(
                "Corrupted NMEA sentence: {:02X?} != {:02X?}",
                checksum_hex_calculated, checksum_hex_given
            )));
        }

        // Pick sentence type
        let mut sentence_type: String = {
            if let Some(i) = sentence.find(',') {
                sentence[0..i].into()
            } else {
                return Err(ParseError::InvalidSentence(format!(
                    "Invalid NMEA sentence: {}",
                    sentence
                )));
            }
        };

        // Identify GNSS system by talker ID.
        let nav_system = {
            if &sentence_type[0..1] == "$" {
                match &sentence_type[1..3] {
                    "GN" => Some(gnss::NavigationSystem::Combination),
                    "GP" => Some(gnss::NavigationSystem::Gps),
                    "GL" => Some(gnss::NavigationSystem::Glonass),
                    "GA" => Some(gnss::NavigationSystem::Galileo),
                    "BD" => Some(gnss::NavigationSystem::Beidou),
                    "GI" => Some(gnss::NavigationSystem::Navic),
                    "QZ" => Some(gnss::NavigationSystem::Qzss),
                    _ => Some(gnss::NavigationSystem::Other),
                }
            } else {
                None
            }
        };
        if nav_system != None {
            // Shorten the GNSS setence types to three letters
            if sentence_type.len() <= 6 {
                sentence_type = format!("${}", &sentence_type[3..6]);
            }
        }

        // Identify AIS station
        let station = {
            if &sentence_type[0..1] == "!" {
                match &sentence_type[1..3] {
                    "AB" => Some(ais::Station::BaseStation),
                    "AD" => Some(ais::Station::DependentAisBaseStation),
                    "AI" => Some(ais::Station::MobileStation),
                    "AN" => Some(ais::Station::AidToNavigationStation),
                    "AR" => Some(ais::Station::AisReceivingStation),
                    "AS" => Some(ais::Station::LimitedBaseStation),
                    "AT" => Some(ais::Station::AisTransmittingStation),
                    "AX" => Some(ais::Station::RepeaterStation),
                    _ => Some(ais::Station::Other),
                }
            } else {
                None
            }
        };
        if station != None {
            // Shorten the AIS setence types to three letters
            if sentence_type.len() <= 6 {
                sentence_type = format!("!{}", &sentence_type[3..6]);
            }
        }

        // Handle sentence types
        match sentence_type.as_str() {
            // $xxGGA - Global Positioning System Fix Data
            "$GGA" => {
                return gnss::gga::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxRMC - Recommended minimum specific GPS/Transit data
            "$RMC" => {
                return gnss::rmc::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxGSA - GPS DOP and active satellites
            "$GSA" => {
                return gnss::gsa::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxGSV - GPS Satellites in view
            "$GSV" => {
                return gnss::gsv::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                    self,
                );
            }
            // $xxVTG - Track made good and ground speed
            "$VTG" => {
                return gnss::vtg::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxGLL - Geographic position, latitude / longitude
            "$GLL" => {
                return gnss::gll::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxALM - Almanac Data
            "$ALM" => {
                return gnss::alm::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxDTM - Datum reference
            "$DTM" => {
                return gnss::dtm::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxMSS - MSK receiver signal
            "$MSS" => {
                return gnss::mss::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxSTN - Multiple Data ID
            "$STN" => {
                return gnss::stn::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }
            // $xxVBW - MSK Receiver Signal
            "$VBW" => {
                return gnss::vbw::handle(
                    sentence.as_str(),
                    nav_system.unwrap_or(gnss::NavigationSystem::Other),
                );
            }

            // Received AIS data from other or own vessel
            "!VDM" | "!VDO" => {
                let own_vessel = sentence_type.as_str() == "!VDO";
                let mut num = 0;
                let mut fragment_count = 0;
                let mut fragment_number = 0;
                let mut message_id = None;
                let mut radio_channel_code = None;
                let mut payload_string: String = "".into();
                for s in sentence.split(",") {
                    match num {
                        1 => {
                            match s.parse::<u8>() {
                                Ok(i) => {
                                    fragment_count = i;
                                }
                                Err(_) => {
                                    return Err(ParseError::InvalidSentence(format!(
                                        "Failed to parse fragment count: {}",
                                        s
                                    )));
                                }
                            };
                        }
                        2 => {
                            match s.parse::<u8>() {
                                Ok(i) => {
                                    fragment_number = i;
                                }
                                Err(_) => {
                                    return Err(ParseError::InvalidSentence(format!(
                                        "Failed to parse fragment count: {}",
                                        s
                                    )));
                                }
                            };
                        }
                        3 => {
                            message_id = s.parse::<u64>().ok();
                        }
                        4 => {
                            // Radio channel code
                            radio_channel_code = Some(s);
                        }
                        5 => {
                            payload_string = s.to_string();
                        }
                        6 => {
                            // fill bits
                        }
                        _ => {}
                    }
                    num += 1;
                }

                // Try parse the payload
                let mut bv: Option<BitVec> = None;
                if fragment_count == 1 {
                    bv = parse_payload(&payload_string).ok();
                } else if fragment_count == 2 {
                    if let Some(msg_id) = message_id {
                        let key1 = make_fragment_key(
                            &sentence_type.to_string(),
                            msg_id,
                            fragment_count,
                            1,
                            radio_channel_code.unwrap_or(""),
                        );
                        let key2 = make_fragment_key(
                            &sentence_type.to_string(),
                            msg_id,
                            fragment_count,
                            2,
                            radio_channel_code.unwrap_or(""),
                        );
                        if fragment_number == 1 {
                            if let Some(p) = self.pull_string(key2.into()) {
                                let mut payload_string_combined = payload_string;
                                payload_string_combined.push_str(p.as_str());
                                bv = parse_payload(&payload_string_combined).ok();
                            } else {
                                self.push_string(key1.into(), payload_string);
                            }
                        } else if fragment_number == 2 {
                            if let Some(p) = self.pull_string(key1.into()) {
                                let mut payload_string_combined = p.clone();
                                payload_string_combined.push_str(payload_string.as_str());
                                bv = parse_payload(&payload_string_combined).ok();
                            } else {
                                self.push_string(key2.into(), payload_string);
                            }
                        } else {
                            warn!(
                                "Unexpected NMEA fragment number: {}/{}",
                                fragment_number, fragment_count
                            );
                        }
                    } else {
                        warn!(
                            "NMEA message_id missing from {} than supported 2",
                            sentence_type
                        );
                    }
                } else {
                    warn!(
                        "NMEA sentence fragment count greater ({}) than supported 2",
                        fragment_count
                    );
                }

                if let Some(bv) = bv {
                    let message_type = pick_u64(&bv, 0, 6);
                    match message_type {
                        // Position report with SOTDMA/ITDMA
                        1 | 2 | 3 => {
                            return ais::vdm_t1t2t3::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Base station report
                        4 => {
                            return ais::vdm_t4::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Ship static voyage related data
                        5 => {
                            return ais::vdm_t5::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Addressed binary message
                        6 => {
                            return ais::vdm_t6::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Binary acknowledge
                        7 => {
                            // TODO: implementation
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                        // Binary broadcast message
                        8 => {
                            // TODO: implementation
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                        // Standard SAR aircraft position report
                        9 => {
                            return ais::vdm_t9::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // UTC and Date inquiry
                        10 => {
                            return ais::vdm_t10::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // UTC and date response
                        11 => {
                            return ais::vdm_t11::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Addressed safety related message
                        12 => {
                            return ais::vdm_t12::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Safety related acknowledge
                        13 => {
                            return ais::vdm_t13::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Safety related broadcast message
                        14 => {
                            return ais::vdm_t14::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Interrogation
                        15 => {
                            return ais::vdm_t15::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Assigned mode command
                        16 => {
                            return ais::vdm_t16::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // GNSS binary broadcast message
                        17 => {
                            return ais::vdm_t17::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Standard class B CS position report
                        18 => {
                            return ais::vdm_t18::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Extended class B equipment position report
                        19 => {
                            return ais::vdm_t19::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Data link management
                        20 => {
                            // TODO: implementation
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                        // Aids-to-navigation report
                        21 => {
                            return ais::vdm_t21::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        // Channel management
                        22 => {
                            // TODO: implementation
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                        // Group assignment command
                        23 => {
                            // TODO: implementation
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                        // Class B CS static data report
                        24 => {
                            return ais::vdm_t24::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                self,
                                own_vessel,
                            );
                        }
                        // Single slot binary message
                        25 => {
                            // TODO: implementation
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                        // Multiple slot binary message
                        26 => {
                            // TODO: implementation
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                        // Long range AIS broadcast message
                        27 => {
                            return ais::vdm_t27::handle(
                                &bv,
                                station.unwrap_or(ais::Station::Other),
                                own_vessel,
                            );
                        }
                        _ => {
                            return Err(ParseError::UnsupportedSentenceType(format!(
                                "Unsupported {} message type: {}",
                                sentence_type, message_type
                            )));
                        }
                    }
                } else {
                    Ok(ParsedMessage::Incomplete)
                }
            }
            _ => Err(ParseError::UnsupportedSentenceType(format!(
                "Unsupported sentence type: {}",
                sentence_type
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_corrupted() {
        // Try a sentence with mismatching checksum
        let mut p = NmeaParser::new();
        assert!(p
            .parse_sentence("!AIVDM,1,1,,A,38Id705000rRVJhE7cl9n;160000,0*41")
            .ok()
            .is_none());
    }

    #[test]
    fn test_parse_missing_checksum() {
        // Try a sentence without checksum
        let mut p = NmeaParser::new();
        assert!(p
            .parse_sentence("!AIVDM,1,1,,A,38Id705000rRVJhE7cl9n;160000,0")
            .ok()
            .is_some());
    }

    #[test]
    fn test_parse_invalid_utc() {
        // Try a sentence with invalite utc
        let mut p = NmeaParser::new();
        assert_eq!(
            p.parse_sentence("!AIVDM,1,1,,B,4028iqT47wP00wGiNbH8H0700`2H,0*13"),
            Err(ParseError::InvalidSentence(String::from(
                "Failed to parse Utc Date from y:4161 m:15 d:31 h:0 m:0 s:0"
            )))
        );
    }

    #[test]
    fn test_nmea_parser() {
        let mut p = NmeaParser::new();

        // String test
        p.push_string("a".into(), "b".into());
        assert_eq!(p.strings_count(), 1);
        p.push_string("c".into(), "d".into());
        assert_eq!(p.strings_count(), 2);
        p.pull_string("a".into());
        assert_eq!(p.strings_count(), 1);
        p.pull_string("c".into());
        assert_eq!(p.strings_count(), 0);

        // VesselStaticData test
        p.push_vsd(1, Default::default());
        assert_eq!(p.vsds_count(), 1);
        p.push_vsd(2, Default::default());
        assert_eq!(p.vsds_count(), 2);
        p.pull_vsd(1);
        assert_eq!(p.vsds_count(), 1);
        p.pull_vsd(2);
        assert_eq!(p.vsds_count(), 0);
    }

    #[test]
    fn test_country() {
        assert_eq!(vsd(230992580).country().unwrap(), "FI");
        assert_eq!(vsd(276009860).country().unwrap(), "EE");
        assert_eq!(vsd(265803690).country().unwrap(), "SE");
        assert_eq!(vsd(273353180).country().unwrap(), "RU");
        assert_eq!(vsd(211805060).country().unwrap(), "DE");
        assert_eq!(vsd(257037270).country().unwrap(), "NO");
        assert_eq!(vsd(227232370).country().unwrap(), "FR");
        assert_eq!(vsd(248221000).country().unwrap(), "MT");
        assert_eq!(vsd(374190000).country().unwrap(), "PA");
        assert_eq!(vsd(412511368).country().unwrap(), "CN");
        assert_eq!(vsd(512003200).country().unwrap(), "NZ");
        assert_eq!(vsd(995126020).country(), None);
        assert_eq!(vsd(2300049).country(), None);
        assert_eq!(vsd(0).country(), None);
    }

    /// Create a `VesselStaticData` with the given MMSI
    fn vsd(mmsi: u32) -> ais::VesselStaticData {
        let mut vsd = ais::VesselStaticData::default();
        vsd.mmsi = mmsi;
        vsd
    }
}
