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

/// Type 10: UTC/Date Inquiry
#[derive(Default, Clone, Debug, PartialEq)]
pub struct UtcDateInquiry { 
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Source MMSI (30 bits)
    pub source_mmsi: u32,

    /// Destination MMSI (30 bits)
    pub destination_mmsi: u32,

}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 10: UTC/Date Inquiry
pub(crate) fn handle(bv: &BitVec, station: Station, own_vessel: bool) -> Result<ParsedSentence, ParseError> {
    return Ok(ParsedSentence::UtcDateInquiry(UtcDateInquiry{
        own_vessel: {
            own_vessel
        },
        station: {
            station
        },
        source_mmsi: {
            pick_u64(&bv, 8, 30) as u32
        },
        destination_mmsi: {
            pick_u64(&bv, 40, 30) as u32
        },
    }));
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type9() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,B,:5MlU41GMK6@,0*6C") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::UtcDateInquiry(udi) => {
                        assert_eq!(udi.source_mmsi, 366814480);
                        assert_eq!(udi.destination_mmsi, 366832740);
                    },
                    ParsedSentence::Incomplete => {
                        assert!(false);
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
}

