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

/// Type 20: Data Link Management Message
#[derive(Clone, Debug, PartialEq)]
pub struct DataLinkManagementMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Interrogation case based on data length
    pub case: InterrogationCase,

    /// Source MMSI (30 bits)
    pub mmsi: u32,

    /// Offset number 1 (12 bits)
    pub offset1: u16,

    /// Reserved offset number (4)
    pub number1: u8,

    /// Allocation timeout in munites (4 bits)
    pub timeout1: u8,

    /// Repeat increment (11 bits)
    pub increment1: u8,

    /// Offset number 2 (12 bits)
    pub offset2: u16,

    /// Reserved offset number (4)
    pub number2: u8,

    /// Allocation timeout in munites (4 bits)
    pub timeout2: u8,

    /// Repeat increment (11 bits)
    pub increment2: u8,

    /// Offset number 3 (12 bits)
    pub offset3: u16,

    /// Reserved offset number (4)
    pub number3: u8,

    /// Allocation timeout in munites (4 bits)
    pub timeout3: u8,

    /// Repeat increment (11 bits)
    pub increment3: u8,

    /// Offset number 4 (12 bits)
    pub offset4: u16,

    /// Reserved offset number (4)
    pub number4: u8,

    /// Allocation timeout in munites (4 bits)
    pub timeout4: u8,

    /// Repeat increment (11 bits)
    pub increment4: u8,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 20: Data Link Management Message
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    let case = InterrogationCase::new(bv);
    Ok(ParsedMessage::DataLinkManagementMessage(
        DataLinkManagementMessage {
            own_vessel,
            station,
            case,
            mmsi: { pick_u64(&bv, 8, 30) as u32 },
            offset1: { pick_u64(&bv, 40, 12) as u16 },
            number1: { pick_u64(&bv, 52, 4) as u8 },
            timeout1: { pick_u64(&bv, 56, 3) as u8 },
            increment1: { pick_u64(&bv, 59, 11) as u8 },
            offset2: { pick_u64(&bv, 70, 12) as u16 },
            number2: { pick_u64(&bv, 82, 4) as u8 },
            timeout2: { pick_u64(&bv, 86, 3) as u8 },
            increment2: { pick_u64(&bv, 89, 11) as u8 },
            offset3: { pick_u64(&bv, 100, 12) as u16 },
            number3: { pick_u64(&bv, 112, 4) as u8 },
            timeout3: { pick_u64(&bv, 116, 3) as u8 },
            increment3: { pick_u64(&bv, 119, 11) as u8 },
            offset4: { pick_u64(&bv, 130, 12) as u16 },
            number4: { pick_u64(&bv, 142, 4) as u8 },
            timeout4: { pick_u64(&bv, 146, 3) as u8 },
            increment4: { pick_u64(&bv, 149, 11) as u8 },
        },
    ))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type20() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,Dh3OvjB8IN>4,0*1D") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::DataLinkManagementMessage(dlmm) => {
                        assert_eq!(dlmm.mmsi, 3669705);
                        assert_eq!(dlmm.offset1, 2182);
                        assert_eq!(dlmm.number1, 5);
                        assert_eq!(dlmm.timeout1, 7);
                        assert_eq!(dlmm.increment1, 225);
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
