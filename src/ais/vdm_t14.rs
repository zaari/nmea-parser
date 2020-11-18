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

/// Type 14: Safety-Related Broadcast Message
#[derive(Default, Clone, Debug, PartialEq)]
pub struct SafetyRelatedBroadcastMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Source MMSI (30 bits)
    pub mmsi: u32,

    /// Text (1-161 ASCII chars)
    pub text: String,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 14: Safety-Related Broadcast Message
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::SafetyRelatedBroadcastMessage(
        SafetyRelatedBroadcastMessage {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            text: { pick_string(&bv, 40, 161) },
        },
    ))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type14() {
        let mut p = NmeaParser::new();

        // First message
        match p.parse_sentence("!AIVDM,1,1,,A,>5?Per18=HB1U:1@E=B0m<L,2*51") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::SafetyRelatedBroadcastMessage(srbm) => {
                        assert_eq!(srbm.mmsi, 351809000);
                        assert_eq!(srbm.text, "RCVD YR TEST MSG");
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
        match p.parse_sentence("!AIVDM,1,1,,A,>3R1p10E3;;R0USCR0HO>0@gN10kGJp,2*7F") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::SafetyRelatedBroadcastMessage(srbm) => {
                        assert_eq!(srbm.mmsi, 237008900);
                        assert_eq!(srbm.text, "EP228 IX48 FG3 DK7 PL56.");
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

        // Third message
        match p.parse_sentence("!AIVDM,1,1,,A,>4aDT81@E=@,2*2E") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::SafetyRelatedBroadcastMessage(srbm) => {
                        assert_eq!(srbm.mmsi, 311764000);
                        assert_eq!(srbm.text, "TEST");
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
