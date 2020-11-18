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

/// Type 12: Addressed Safety-Related Message
#[derive(Default, Clone, Debug, PartialEq)]
pub struct AddressedSafetyRelatedMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Source MMSI (30 bits)
    pub source_mmsi: u32,

    /// Sequence number (2 bits)
    pub sequence_number: u8,

    /// Destination MMSI (30 bits)
    pub destination_mmsi: u32,

    /// Retransmit flag (1 bit)
    pub retransmit_flag: bool,

    /// Text (936 bits; 1-156 chars)
    pub text: String,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 12: Addressed Safety-Related Message
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::AddressedSafetyRelatedMessage(
        AddressedSafetyRelatedMessage {
            own_vessel: { own_vessel },
            station: { station },
            source_mmsi: { pick_u64(&bv, 8, 30) as u32 },
            sequence_number: { pick_u64(&bv, 38, 2) as u8 },
            destination_mmsi: { pick_u64(&bv, 40, 30) as u32 },
            retransmit_flag: { pick_u64(&bv, 70, 1) != 0 },
            text: { pick_string(&bv, 72, 156) },
        },
    ))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type12() {
        // First message
        let mut p = NmeaParser::new();
        match p.parse_sentence(
            "!AIVDM,1,1,,A,<02:oP0kKcv0@<51C5PB5@?BDPD?P:?2?EB7PDB16693P381>>5<PikP,0*37",
        ) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::AddressedSafetyRelatedMessage(asrm) => {
                        assert_eq!(asrm.source_mmsi, 2275200);
                        assert_eq!(asrm.sequence_number, 0);
                        assert_eq!(asrm.destination_mmsi, 215724000);
                        assert_eq!(asrm.retransmit_flag, false);
                        assert_eq!(asrm.text, "PLEASE REPORT TO JOBOURG TRAFFIC CHANNEL 13");
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

        // Second message
        match p.parse_sentence(
            "!AIVDM,1,1,,A,<CR3B@<0TO3j5@PmkiP31BCPphPDB13;CPihkP=?D?PmP3B5GPpn,0*3A",
        ) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::AddressedSafetyRelatedMessage(asrm) => {
                        assert_eq!(asrm.source_mmsi, 237032000);
                        assert_eq!(asrm.sequence_number, 3);
                        assert_eq!(asrm.destination_mmsi, 2391100);
                        assert_eq!(asrm.retransmit_flag, true);
                        assert_eq!(asrm.text, "EP 531 CARS 80 TRACKS 103 MOTO 5 CREW 86");
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
