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

/// Type 22: Channel Management
#[derive(Default, Clone, Debug, PartialEq)]
pub struct ChannelManagement {
    /// True if the data is about own vessel, false if about other.
    pub own_vessel: bool,

    /// AIS station type.
    pub station: Station,

    /// User ID (30 bits)
    pub mmsi: u32,

    /// Channel A number (12 bits).
    pub channel_a: u16,

    /// Channel B number (12 bits).
    pub channel_b: u16,

    /// TxRx mode:
    /// 0 = TxA/TxB, RxA/RxB (default)
    /// 1 = TxA, RxA/RxB
    /// 2 = TxB, RxA/RxB
    /// 3 = Reserved for future use
    pub txrx: u8,

    /// Power level to be used:
    /// 0 = low,
    /// 1 = high
    pub power: bool,

    /// Northeast latitude to 0.1 minutes.
    pub ne_lat: Option<f64>,

    /// Northeast longitude to 0.1 minutes.
    pub ne_lon: Option<f64>,

    /// Southwest latitude to 0.1 minutes.
    pub sw_lat: Option<f64>,

    /// Southwest longitude to 0.1 minutes.
    pub sw_lon: Option<f64>,

    /// MMSI of destination 1 (30 bits).
    pub dest1_mmsi: Option<u32>,

    /// MMSI of destination 2 (30 bits).
    pub dest2_mmsi: Option<u32>,

    /// Addressed:
    /// false = broadcast,
    /// true = addressed
    pub addressed: bool,

    /// Channel A band:
    /// false = default,
    /// true = 12.5 kHz
    pub channel_a_band: bool,

    /// Channel B band:
    /// false = default,
    /// true = 12.5 kHz
    pub channel_b_band: bool,

    /// Size of transitional zone (3 bits).
    pub zonesize: u8,
}

// -------------------------------------------------------------------------------------------------

/// AIS VDM/VDO type 22: Channel Management
pub(crate) fn handle(
    bv: &BitVec,
    station: Station,
    own_vessel: bool,
) -> Result<ParsedMessage, ParseError> {
    let addressed = pick_u64(bv, 139, 1) != 0;
    Ok(ParsedMessage::ChannelManagement(ChannelManagement {
        own_vessel: { own_vessel },
        station: { station },
        mmsi: { pick_u64(bv, 8, 30) as u32 },
        channel_a: { pick_u64(bv, 40, 12) as u16 },
        channel_b: { pick_u64(bv, 52, 12) as u16 },
        txrx: { pick_u64(bv, 64, 4) as u8 },
        power: { pick_u64(bv, 68, 1) != 0 },
        ne_lat: {
            if !addressed {
                Some(pick_i64(bv, 87, 17) as f64 / 600.0)
            } else {
                None
            }
        },
        ne_lon: {
            if !addressed {
                Some(pick_i64(bv, 69, 18) as f64 / 600.0)
            } else {
                None
            }
        },
        sw_lat: {
            if !addressed {
                Some(pick_i64(bv, 122, 17) as f64 / 600.0)
            } else {
                None
            }
        },
        sw_lon: {
            if !addressed {
                Some(pick_i64(bv, 104, 18) as f64 / 600.0)
            } else {
                None
            }
        },
        dest1_mmsi: {
            if addressed {
                Some(pick_u64(bv, 69, 30) as u32)
            } else {
                None
            }
        },
        dest2_mmsi: {
            if addressed {
                Some(pick_u64(bv, 104, 30) as u32)
            } else {
                None
            }
        },
        addressed: { pick_u64(bv, 139, 1) != 0 },
        channel_a_band: { pick_u64(bv, 140, 1) != 0 },
        channel_b_band: { pick_u64(bv, 141, 1) != 0 },
        zonesize: { pick_u64(bv, 142, 3) as u8 },
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type22() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("!AIVDM,1,1,,A,F030ot22N2P6aoQbhe4736L20000,0*1A") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::ChannelManagement(cm) => {
                        assert_eq!(cm.mmsi, 3160048);
                        assert_eq!(cm.channel_a, 2087);
                        assert_eq!(cm.channel_b, 2088);
                        assert_eq!(cm.txrx, 0);
                        assert_eq!(cm.power, false);
                        assert::close(cm.ne_lat.unwrap_or(0.0), 45.55, 0.01);
                        assert::close(cm.ne_lon.unwrap_or(0.0), -73.50, 0.01);
                        assert::close(cm.sw_lat.unwrap_or(0.0), 42.33, 0.01);
                        assert::close(cm.sw_lon.unwrap_or(0.0), -80.17, 0.01);
                        assert_eq!(cm.addressed, false);
                        assert_eq!(cm.channel_a_band, false);
                        assert_eq!(cm.channel_b_band, false);
                        assert_eq!(cm.zonesize, 4);
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
