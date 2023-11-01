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

/// Type 17: DGNSS Broadcast Binary Message.
#[derive(Clone, Debug, PartialEq)]
pub struct DgnssBroadcastBinaryMessage {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// Source MMSI (30 bits)
    pub mmsi: u32,

    /// Latitude (17 bits)
    pub latitude: Option<f64>,

    /// Longitude (18 bits)
    pub longitude: Option<f64>,

    /// Payload (80-815 bits). Note that it appears to be tied to the now obsolete RTCM2 protocol.
    pub payload: BitVec,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 17: DGNSS Broadcast Binary Message
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    Ok(ParsedMessage::DgnssBroadcastBinaryMessage(
        DgnssBroadcastBinaryMessage {
            own_vessel: { own_vessel },
            station: { station },
            mmsi: { pick_u64(bv, 8, 30) as u32 },
            latitude: {
                let lat_raw = pick_i64(bv, 58, 17) as i32;
                if lat_raw != 0xd548 {
                    Some((lat_raw as f64) / 600.0)
                } else {
                    None
                }
            },
            longitude: {
                let lon_raw = pick_i64(bv, 40, 18) as i32;
                if lon_raw != 0x1a838 {
                    Some((lon_raw as f64) / 600.0)
                } else {
                    None
                }
            },
            payload: bv.iter().skip(80).collect(),
        },
    ))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type17() {
        let mut p = NmeaParser::new();
        match p.parse_sentence(
            "!AIVDM,2,1,5,A,A02VqLPA4I6C07h5Ed1h<OrsuBTTwS?r:C?w`?la<gno1RTRwSP9:BcurA8a,0*3A",
        ) {
            Ok(ps) => match ps {
                ParsedMessage::Incomplete => {
                    assert!(true);
                }
                _ => {
                    assert!(false);
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
        match p.parse_sentence("!AIVDM,2,2,5,A,:Oko02TSwu8<:Jbb,0*11") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::DgnssBroadcastBinaryMessage(i) => {
                        assert_eq!(i.mmsi, 2734450);
                        assert::close(i.latitude.unwrap_or(0.0), 59.987, 0.001);
                        assert::close(i.longitude.unwrap_or(0.0), 29.130, 0.001);
                        assert_eq!(i.payload.len(), 376);
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
