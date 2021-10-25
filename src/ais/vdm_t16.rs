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

/// Type 16: Assignment Mode Command
#[derive(Clone, Debug, PartialEq)]
pub struct AssignmentModeCommand {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    // When the message is 96 bits long it is interpreted as an assignment for a single station,
    // When the message is 144 bits long it is interpreted as a channel assignled for two stations.
    pub assigned_for_single_station: bool,

    /// Source MMSI (30 bits)
    pub mmsi: u32,

    /// Destination A MMSI (30 bits)
    pub mmsi1: u32,

    /// Offset A
    pub offset1: u16,

    /// Increment A
    pub increment1: u16,

    /// Destination B MMSI (30 bits)
    pub mmsi2: Option<u32>,

    /// Offset B
    pub offset2: Option<u16>,

    /// Increment B
    pub increment2: Option<u16>,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 16: Assignment Mode Command
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    let single = bv.len() < 144;
    Ok(ParsedMessage::AssignmentModeCommand(
        AssignmentModeCommand {
            own_vessel: { own_vessel },
            station: { station },
            assigned_for_single_station: { single },
            mmsi: { pick_u64(bv, 8, 30) as u32 },
            mmsi1: { pick_u64(bv, 40, 30) as u32 },
            offset1: { pick_u64(bv, 70, 12) as u16 },
            increment1: { pick_u64(bv, 82, 10) as u16 },
            mmsi2: {
                if single {
                    None
                } else {
                    Some(pick_u64(bv, 92, 30) as u32)
                }
            },
            offset2: {
                if single {
                    None
                } else {
                    Some(pick_u64(bv, 122, 12) as u16)
                }
            },
            increment2: {
                if single {
                    None
                } else {
                    Some(pick_u64(bv, 134, 10) as u16)
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
    fn test_parse_vdm_type16() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,@01uEO@mMk7P<P00,0*18") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::AssignmentModeCommand(i) => {
                        assert_eq!(i.mmsi, 2053501);
                        assert_eq!(i.mmsi1, 224251000);
                        assert_eq!(i.offset1, 200);
                        assert_eq!(i.increment1, 0);
                        assert_eq!(i.mmsi2, None);
                        assert_eq!(i.offset2, None);
                        assert_eq!(i.increment2, None);
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
