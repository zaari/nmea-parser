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

/// AIS VDM/VDO type 4: Base Station Report
#[derive(Default, Clone, Debug, PartialEq)]
pub struct BinaryAddressedMessage { 
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// User ID (2 bits)
    pub sequence_number: u8,

    /// User ID (30 bits)
    pub destination_mmsi: u32,

    /// Retransmit flag
    pub retransmit_flag: bool,

    /// Designated area code, DAC (10 bits)
    pub dac: u16,

    /// Functional ID, FID (6 bits)
    pub fid: u8,

    // TODO: data (depending on DAC and FID
}

impl LatLon for BinaryAddressedMessage {
    fn latitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }

    fn longitude(&self) -> Option<f64> {
        None // TODO: depends on data
    }
}

// -------------------------------------------------------------------------------------------------

#[doc(hidden)]
/// AIS VDM/VDO types 6: Binary Addressed Message
pub fn handle(bv: &BitVec, station: Station, own_vessel: bool) -> Result<ParsedSentence, ParseError> {
    return Ok(ParsedSentence::BinaryAddressedMessage(BinaryAddressedMessage{
        own_vessel: {
            own_vessel
        },
        station: {
            station
        },
        mmsi: {
            pick_u64(&bv, 8, 30) as u32
        },
        sequence_number: {
            pick_u64(&bv, 38, 2) as u8
        },
        destination_mmsi: {
            pick_u64(&bv, 40, 30) as u32
        },
        retransmit_flag: {
            pick_u64(&bv, 70, 1) != 0
        },
        dac: {
            pick_u64(&bv, 72, 10) as u16
        },
        fid: {
            pick_u64(&bv, 82, 6) as u8
        }

        // TODO: data (depending on DAC and FID
    }));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type5() {
        match parse_sentence("!AIVDM,1,1,,B,6B?n;be:cbapalgc;i6?Ow4,2*4A", 
                              &mut NmeaStore::new()) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::BinaryAddressedMessage(bam) => {
                        assert_eq!(bam.mmsi, 150834090);
                        assert_eq!(bam.sequence_number, 3);
                        assert_eq!(bam.destination_mmsi, 313240222);
                        assert_eq!(bam.retransmit_flag, false);
                        assert_eq!(bam.dac, 669);
                        assert_eq!(bam.fid, 11);
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

