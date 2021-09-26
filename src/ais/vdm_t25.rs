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

use super::*;

// -------------------------------------------------------------------------------------------------

/// Type 25: Single Slot Binary Message
#[derive(Default, Clone, Debug, PartialEq)]
pub struct SingleSlotBinaryMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// When 'addressed' flag is on this field contains the parsed destination MMSI.
    pub dest_mmsi: Option<u32>,

    /// When 'addressed' flag is off and 'structured' flag on this field contains
    /// application ID which consists of 10-bit DAC and 6-bit FID as in message types 6 and 8.
    pub app_id: Option<u16>,

    /// Data field of length 0-128 bits.
    pub data: BitVec,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 25: Single Slot Binary Message
#[allow(clippy::collapsible_if)]
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    let addressed = pick_u64(&bv, 38, 1) != 0;
    let structured = pick_u64(&bv, 39, 1) != 0;

    Ok(ParsedMessage::SingleSlotBinaryMessage(
        SingleSlotBinaryMessage {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            dest_mmsi: {
                if addressed {
                    Some(pick_u64(&bv, 40, 30) as u32)
                } else {
                    None
                }
            },
            app_id: {
                if addressed {
                    None
                } else {
                    if structured {
                        Some(pick_u64(&bv, 70, 16) as u16)
                    } else {
                        None
                    }
                }
            },
            data: {
                if addressed {
                    BitVec::from_bitslice(&bv[70..max(70, bv.len())])
                } else {
                    if structured {
                        BitVec::from_bitslice(&bv[86..max(86, bv.len())])
                    } else {
                        BitVec::from_bitslice(&bv[40..max(40, bv.len())])
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
    fn test_parse_vdm_type25() {
        let mut p = NmeaParser::new();

        // Test 1
        match p.parse_sentence("!AIVDM,1,1,,A,I6SWo?8P00a3PKpEKEVj0?vNP<65,0*73") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::SingleSlotBinaryMessage(ssbm) => {
                        assert_eq!(ssbm.mmsi, 440006460);
                        assert_eq!(ssbm.dest_mmsi, Some(134218384));
                        assert_eq!(ssbm.app_id, None);
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

        // Test 2
        match p.parse_sentence("!AIVDM,1,1,,A,I8IRGB40QPPa0:<HP::V=gwv0l48,0*0E") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::SingleSlotBinaryMessage(ssbm) => {
                        assert_eq!(ssbm.mmsi, 563648328);
                        assert_eq!(ssbm.dest_mmsi, None);
                        assert_eq!(ssbm.app_id, Some(16424));
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

        // Test 3
        match p.parse_sentence("!AIVDM,1,1,,A,I6SWVNP001a3P8FEKNf=Qb0@00S8,0*6B") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::SingleSlotBinaryMessage(ssbm) => {
                        assert_eq!(ssbm.mmsi, 440002170);
                        assert_eq!(ssbm.dest_mmsi, None);
                        assert_eq!(ssbm.app_id, None);
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
