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

/// Type 15: Interrogation
#[derive(Default, Clone, Debug, PartialEq)]
pub struct Interrogation {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Source MMSI (30 bits)
    pub mmsi: u32,

    /// Interrogated MMSI (30 bits)
    pub mmsi1: u32,

    /// First message type (6 bits)
    pub type1_1: u8,

    /// First slot offset (12 bits)
    pub offset1_1: u16,

    /// Second message type (6 bits)
    pub type1_2: Option<u8>,

    /// Second slot offset (12 bits)
    pub offset1_2: Option<u16>,

    /// Interrogated MMSI (30 bits)
    pub mmsi2: Option<u32>,

    /// First message type (6 bits)
    pub type2_1: Option<u8>,

    /// First slot offset (12 bits)
    pub offset2_1: Option<u16>,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 15: Interrogation
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    let case = {
        let len = bv.len();
        if len >= 160 {
            if pick_u64(&bv, 90, 18) == 0 {
                3 // Case 3 (160 bits but without second type and second slot)
            } else {
                4 // Case 4 (160 bits)
            }
        } else if len >= 110 {
            2 // Case 2 (110 bits))
        } else {
            1 // Case 1 (88 bits)
        }
    };

    return Ok(ParsedMessage::Interrogation(Interrogation {
        own_vessel: { own_vessel },
        station: { station },
        mmsi: { pick_u64(&bv, 8, 30) as u32 },
        mmsi1: { pick_u64(&bv, 40, 30) as u32 },
        type1_1: { pick_u64(&bv, 70, 6) as u8 },
        offset1_1: { pick_u64(&bv, 76, 12) as u16 },
        type1_2: match case {
            2 => Some(pick_u64(&bv, 90, 6) as u8),
            _ => None,
        },
        offset1_2: match case {
            2 => Some(pick_u64(&bv, 96, 12) as u16),
            _ => None,
        },
        mmsi2: match case {
            3 => Some(pick_u64(&bv, 110, 30) as u32),
            _ => None,
        },
        type2_1: match case {
            4 => Some(pick_u64(&bv, 140, 6) as u8),
            _ => None,
        },
        offset2_1: match case {
            4 => Some(pick_u64(&bv, 146, 12) as u16),
            _ => None,
        },
    }));
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type15() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,B,?h3Ovn1GP<K0<P@59a0,2*04") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Interrogation(i) => {
                        assert_eq!(i.mmsi, 3669720);
                        assert_eq!(i.mmsi1, 367014320);
                        assert_eq!(i.type1_1, 3);
                        assert_eq!(i.offset1_1, 516);
                        assert_eq!(i.type1_2, Some(5));
                        assert_eq!(i.offset1_2, Some(617));
                        assert_eq!(i.mmsi2, None);
                        assert_eq!(i.type2_1, None);
                        assert_eq!(i.offset2_1, None);
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
