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

/// Type 13: Safety-Related Acknowledgment
#[derive(Default, Clone, Debug, PartialEq)]
pub struct SafetyRelatedAcknowledgement {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Source MMSI (30 bits)
    pub source_mmsi: u32,

    /// MMSI number 1 (30 bits)
    pub mmsi1: u32,

    /// MMSI sequence
    pub mmsi1_seq: u8,

    /// MMSI number 2 (30 bits)
    pub mmsi2: u32,

    /// MMSI sequence
    pub mmsi2_seq: u8,

    /// MMSI number 3 (30 bits)
    pub mmsi3: u32,

    /// MMSI sequence
    pub mmsi3_seq: u8,

    /// MMSI number 4 (30 bits)
    pub mmsi4: u32,

    /// MMSI sequence
    pub mmsi4_seq: u8,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 13: Safety-Related Acknowledgment
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedSentence, ParseError> {
    return Ok(ParsedSentence::SafetyRelatedAcknowledgement(
        SafetyRelatedAcknowledgement {
            own_vessel: { own_vessel },
            station: { station },
            source_mmsi: { pick_u64(&bv, 8, 30) as u32 },
            mmsi1: { pick_u64(&bv, 40, 30) as u32 },
            mmsi1_seq: { pick_u64(&bv, 70, 2) as u8 },
            mmsi2: { pick_u64(&bv, 72, 30) as u32 },
            mmsi2_seq: { pick_u64(&bv, 102, 2) as u8 },
            mmsi3: { pick_u64(&bv, 104, 30) as u32 },
            mmsi3_seq: { pick_u64(&bv, 134, 2) as u8 },
            mmsi4: { pick_u64(&bv, 136, 30) as u32 },
            mmsi4_seq: { pick_u64(&bv, 166, 2) as u8 },
        },
    ));
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type13() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,=39UOj0jFs9R,0*65") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::SafetyRelatedAcknowledgement(sra) => {
                        assert_eq!(sra.source_mmsi, 211378120);
                        assert_eq!(sra.mmsi1, 211217560);
                        assert_eq!(sra.mmsi1_seq, 2);
                        assert_eq!(sra.mmsi2, 0);
                        assert_eq!(sra.mmsi2_seq, 0);
                        assert_eq!(sra.mmsi3, 0);
                        assert_eq!(sra.mmsi3_seq, 0);
                        assert_eq!(sra.mmsi4, 0);
                        assert_eq!(sra.mmsi4_seq, 0);
                    }
                    ParsedSentence::Incomplete => {
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
