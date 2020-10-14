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
//! This crate aims to cover the most important AIS and GNSS sentences. 
//! Supports AIS class A and B types. Identifies GPS, GLONASS, Galileo, BeiDou, 
//! Navic and QZSS satellite systems. 
//!

#![allow(dead_code)]

#[macro_use] extern crate log;
extern crate env_logger;

extern crate chrono;

use std::collections::{HashMap};
use bitvec::prelude::*;
use chrono::{DateTime};
use chrono::prelude::*;

pub mod ais;
pub mod gnss;
mod error;
mod util;

use util::*;
pub use error::{ParseError};

// -------------------------------------------------------------------------------------------------
/// Result from function `parse_sentence`
#[derive(Clone, Debug, PartialEq)]
pub enum ParsedSentence {
    /// The given sentence is only part of multi-sentence message and we need more data to
    /// create the actual result. State is stored in `NmeaStore` object.
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
}

// -------------------------------------------------------------------------------------------------

/// Used to store partial sentences between `parse_sentence` function calls
pub struct NmeaStore {
    saved_fragments: HashMap<String, String>,
    saved_vsds: HashMap<u32, ais::VesselStaticData>,
}

impl NmeaStore {
    /// Default constructor.
    pub fn new() -> NmeaStore {
        NmeaStore {
            saved_fragments: HashMap::new(),
            saved_vsds:      HashMap::new(),
        }
    }
    
    /// Push string-to-string mapping to store.
    pub fn push_string(&mut self, key: String, value: String) {
        self.saved_fragments.insert(key, value);
    }

    /// Pull string-to-string mapping by key from store.
    pub fn pull_string(&mut self, key: String) -> Option<String> {
        self.saved_fragments.remove(&key)
    }

    /// Tests whether the given string-to-string mapping exists in the store.
    pub fn contains_key(&mut self, key: String) -> bool {
        self.saved_fragments.contains_key(&key)
    }

    /// Return number of string-to-string mappings stored.
    pub fn strings_count(&self) -> usize {
        self.saved_fragments.len()
    }

    /// Push MMSI-to-VesselStaticData mapping to store.
    pub fn push_vsd(&mut self, mmsi: u32, vsd: ais::VesselStaticData) {
        self.saved_vsds.insert(mmsi, vsd);
    }
    
    /// Pull MMSI-to-VesselStaticData mapping from store.
    pub fn pull_vsd(&mut self, mmsi: u32) -> Option<ais::VesselStaticData> {
        self.saved_vsds.remove(&mmsi)
    }

    /// Return number of MMSI-to-VesselStaticData mappings in store.    
    pub fn vsds_count(&self) -> usize {
        self.saved_vsds.len()
    }
}

// -------------------------------------------------------------------------------------------------

/// Provides access to geographical position in the implementing struct.
pub trait LatLon {
    /// Returns the latitude of the position contained by the object. If the position is not
    /// available returns None.
    fn latitude(&self) -> Option<f64>;

    /// Returns the longitude of the position contained by the object. If the position is not
    /// available returns None.
    fn longitude(&self) -> Option<f64>;
}

// -------------------------------------------------------------------------------------------------

/// Parses NMEA sentence into `ParsedSentence` enum. If the given sentence is part of 
/// multipart message, the state is saved into `store` object and `ParsedSentence::Incomplete` 
/// returned. The actual result is returned when all the parts have been provided for the function.
pub fn parse_sentence(sentence: &str, nmea_store: &mut NmeaStore) -> Result<ParsedSentence, ParseError> {
    // Calculace NMEA checksum and compare it to the given one. Also, remove the checksum part
    // from the sentence to simplify next processing steps.
    let mut checksum = 0;
    let (sentence, checksum_hex_given) = { 
        if let Some(pos) = sentence.rfind('*') {
            (sentence[0..pos].to_string(), sentence[(pos+1)..sentence.len()].to_string())
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
        return Err(ParseError::CorruptedSentence(
                   format!("Corrupted NMEA sentence: {:02X?} != {:02X?}", 
                           checksum_hex_calculated, checksum_hex_given)));
    }
    
    // Pick sentence type
    let mut sentence_type: String = {
        if let Some(i) = sentence.find(',') {
            sentence[0..i].into()
        } else {
            return Err(ParseError::InvalidSentence(format!("Invalid NMEA sentence: {}", sentence)));
        }
    };

    // Recognize GNSS system by talker ID.
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

    // Recognize AIS station
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
            return gnss::gga::handle(sentence.as_str(), nav_system.unwrap_or(gnss::NavigationSystem::Other));
        },
        // $xxRMC - Recommended minimum specific GPS/Transit data
        "$RMC" => {
            return gnss::rmc::handle(sentence.as_str(), nav_system.unwrap_or(gnss::NavigationSystem::Other));
        },
        // $xxGSA - GPS DOP and active satellites 
        "$GSA" => {
            return gnss::gsa::handle(sentence.as_str(), nav_system.unwrap_or(gnss::NavigationSystem::Other));
        },
        // $xxGSV - GPS Satellites in view
        "$GSV" => {
            return gnss::gsv::handle(sentence.as_str(), nav_system.unwrap_or(gnss::NavigationSystem::Other), 
                                    nmea_store);
        },
        // $xxVTG - Track made good and ground speed
        "$VTG" => {
            return gnss::vtg::handle(sentence.as_str(), nav_system.unwrap_or(gnss::NavigationSystem::Other), 
                                    nmea_store);
        },
        // $xxGLL - Geographic position, latitude / longitude
        "$GLL" => {
            return gnss::gll::handle(sentence.as_str(), nav_system.unwrap_or(gnss::NavigationSystem::Other), 
                                    nmea_store);
        },


        // $xxALM - Almanac Data
        "$ALM" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxHDT - Heading, True
        "$HDT" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxTRF - Transit Fix Data
        "$TRF" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxSTN - Multiple Data ID
        "$STN" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxVBW - Dual Ground / Water Speed
        "$VBW" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxXTC - Cross track error
        "$XTC" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxXTE - Cross-track error, Measured
        "$XTE" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxZDA - Date & Time
        "$ZDA" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },



        // $xxBOD Bearing Origin to Destination 
        "$BOD" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },
        // $xxRMA - Recommended minimum specific Loran-C data
        "$RMA" => {
            return Err(ParseError::UnsupportedSentenceType(
                       format!("Unimplemented NMEA sentence: {}", sentence_type))); // TODO
        },


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
                            Ok(i) => { fragment_count = i; },
                            Err(_) => { 
                                return Err(ParseError::InvalidSentence(
                                           format!("Failed to parse fragment count: {}", s))); 
                            }
                        };
                    },
                    2 => {
                        match s.parse::<u8>() {
                            Ok(i) => { fragment_number = i; },
                            Err(_) => { 
                                return Err(ParseError::InvalidSentence(
                                           format!("Failed to parse fragment count: {}", s))); 
                            }
                        };
                    },
                    3 => {
                        message_id = s.parse::<u64>().ok();
                    },
                    4 => {
                        // Radio channel code
                        radio_channel_code = Some(s);
                    },
                    5 => {
                        payload_string = s.to_string();
                    },
                    6 => {
                        // fill bits
                    },
                    _ => {
                    }
                }
                num += 1;
            }

            // Try parse the payload
            let mut bv: Option<BitVec> = None;
            if fragment_count == 1 {
                bv = parse_payload(&payload_string).ok();
            } else if fragment_count == 2 {
                if let Some(msg_id) = message_id {
                    let key1 = make_fragment_key(&sentence_type.to_string(), msg_id, fragment_count, 
                                                 1, radio_channel_code.unwrap_or(""));
                    let key2 = make_fragment_key(&sentence_type.to_string(), msg_id, fragment_count, 
                                                 2, radio_channel_code.unwrap_or(""));
                    if fragment_number == 1 {
                        if let Some(p) = nmea_store.pull_string(key2.into()) {
                            let mut payload_string_combined = payload_string;
                            payload_string_combined.push_str(p.as_str());
                            bv = parse_payload(&payload_string_combined). ok();
                        } else {
                            nmea_store.push_string(key1.into(), payload_string);
                        }
                    } else if fragment_number == 2 {
                        if let Some(p) = nmea_store.pull_string(key1.into()) {
                            let mut payload_string_combined = p.clone();
                            payload_string_combined.push_str(payload_string.as_str());
                            bv = parse_payload(&payload_string_combined).ok();
                        } else {
                            nmea_store.push_string(key2.into(), payload_string);
                        }
                    } else {
                        warn!("Unexpected NMEA fragment number: {}/{}", fragment_number, fragment_count);
                    }
                } else {
                    warn!("NMEA message_id missing from {} than supported 2", sentence_type);
                }
            } else {
                warn!("NMEA sentence fragment count greater ({}) than supported 2", fragment_count);
            }

            if let Some(bv) = bv {
                let message_type = pick_u64(&bv, 0, 6);
                match message_type {
                    // Position Report with SOTDMA/ITDMA
                    1 | 2 | 3 => {
                        return ais::vdm_t1t2t3::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                      own_vessel);
                    },
                    // Base Station Report
                    4 => {
                        return ais::vdm_t4::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                   own_vessel);
                    },
                    // Ship static voyage related data
                    5 => {
                        return ais::vdm_t5::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                  own_vessel);
                    },
                    // Addressed Binary Message 
                    6 => {
                        return ais::vdm_t6::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                  own_vessel);
                    },
                    // Binary Acknowledge
                    7 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Binary Broadcast Message 
                    8 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Standard SAR Aircraft position report 
                    9 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // UTC and Date inquiry 
                    10 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // UTC and Date response 
                    11 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Addressed safety related message 
                    12 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Safety related Acknowledge 
                    13 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Safety related Broadcast Message 
                    14 => {
                        // TODO: implementation (Class B)
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Interrogation 
                    15 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Assigned Mode Command 
                    16 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // GNSS Binary Broadcast Message  
                    17 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Standard Class B CS Position Report 
                    18 => {
                        return ais::vdm_t18::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                   own_vessel);
                    },
                    // Extended Class B Equipment Position Report
                    19 => {
                        return ais::vdm_t19::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                   own_vessel);
                    },
                    // Data Link Management 
                    20 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Aids-to-navigation Report 
                    21 => {
                        return ais::vdm_t21::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                   own_vessel);
                    },
                    // Channel Management 
                    22 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Group Assignment Command 
                    23 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Class B CS Static Data Report
                    24 => {
                        return ais::vdm_t24::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                    nmea_store, own_vessel);
                    },
                    // Single Slot Binary Message
                    25 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Multiple Slot Binary Message
                    26 => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    },
                    // Long Range AIS Broadcast message
                    27 => {
                        return ais::vdm_t27::handle(&bv, station.unwrap_or(ais::Station::Other), 
                                                    own_vessel);
                    },
                    _ => {
                        // TODO: implementation
                        return Err(ParseError::UnsupportedSentenceType(
                                   format!("Unsupported {} message type: {}", 
                                           sentence_type, message_type)));
                    }
                }
            } else {
                Ok(ParsedSentence::Incomplete)
            }
        },
        _ => {
            Err(ParseError::UnsupportedSentenceType(
                format!("Unsupported sentence type: {}", sentence_type)))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_corrupted() {
        // Try a sentence with mismatching checksum
        assert!(parse_sentence("!AIVDM,1,1,,A,38Id705000rRVJhE7cl9n;160000,0*41", 
                                &mut NmeaStore::new()).ok().is_none());
    }

    #[test]
    fn test_parse_missing_checksum() {
        // Try a sentence without checksum
        assert!(parse_sentence("!AIVDM,1,1,,A,38Id705000rRVJhE7cl9n;160000,0", 
                                &mut NmeaStore::new()).ok().is_some());
    }

    #[test]
    fn test_nmea_store() {
        let mut store = NmeaStore::new();
        
        // String test
        store.push_string("a".into(), "b".into());
        assert_eq!(store.strings_count(), 1);
        store.push_string("c".into(), "d".into());
        assert_eq!(store.strings_count(), 2);
        store.pull_string("a".into());
        assert_eq!(store.strings_count(), 1);
        store.pull_string("c".into());
        assert_eq!(store.strings_count(), 0);
        
        // VesselStaticData test
        store.push_vsd(1, Default::default());
        assert_eq!(store.vsds_count(), 1);
        store.push_vsd(2, Default::default());
        assert_eq!(store.vsds_count(), 2);
        store.pull_vsd(1);
        assert_eq!(store.vsds_count(), 1);
        store.pull_vsd(2);
        assert_eq!(store.vsds_count(), 0);
    }

    #[test]
    fn test_mmsi_to_country_code_conversion() {
        let mut vsd = ais::VesselStaticData::default();
        
        vsd.mmsi = 230992580; assert_eq!(vsd.country().unwrap(), "FI");
        vsd.mmsi = 276009860; assert_eq!(vsd.country().unwrap(), "EE");
        vsd.mmsi = 265803690; assert_eq!(vsd.country().unwrap(), "SE");
        vsd.mmsi = 273353180; assert_eq!(vsd.country().unwrap(), "RU");
        vsd.mmsi = 211805060; assert_eq!(vsd.country().unwrap(), "DE");
        vsd.mmsi = 257037270; assert_eq!(vsd.country().unwrap(), "NO");
        vsd.mmsi = 227232370; assert_eq!(vsd.country().unwrap(), "FR");
        vsd.mmsi = 248221000; assert_eq!(vsd.country().unwrap(), "MT");
        vsd.mmsi = 374190000; assert_eq!(vsd.country().unwrap(), "PA");
        vsd.mmsi = 412511368; assert_eq!(vsd.country().unwrap(), "CN");
        vsd.mmsi = 512003200; assert_eq!(vsd.country().unwrap(), "NZ");
        vsd.mmsi = 995126020; assert_eq!(vsd.country(), None);
        vsd.mmsi =   2300049; assert_eq!(vsd.country(), None);
    }

}

