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

/// ALM - GPS Almanac Data
#[derive(Clone, Debug, PartialEq)]
pub struct AlmData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Satellite PRN number (1-32)
    pub prn: Option<u8>,

    /// GPS week number (0-1023).
    pub week_number: Option<u16>,

    /// Satellite health (bits 17-24)
    pub health_bits: Option<u8>,

    /// Eccentricity
    pub eccentricity: Option<u16>,

    /// Reference time of almanac (TOA)
    pub reference_time: Option<u8>,

    /// Satellite inclination angle (sigma)
    pub sigma: Option<u16>,

    /// Rate of right ascension (omega dot)
    pub omega_dot: Option<u16>,

    /// Square root of semi-major axis (root a)
    pub root_a: Option<u32>,

    /// Argument of perigee (omega)
    pub omega: Option<u32>,

    /// Ascending node longitude (omega I)
    pub omega_o: Option<u32>,

    /// Mean anomaly (mo)
    pub mo: Option<u32>,

    /// Clock parameter 0 (af0)
    pub af0: Option<u16>,

    /// Clock parameter 1 (af0)
    pub af1: Option<u16>,
}

// -------------------------------------------------------------------------------------------------

/// xxALM: Global Positioning System Fix Data
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Alm(AlmData {
        source: nav_system,
        prn: pick_hex_field(&split, 3)?,
        week_number: {
            if let Some(wk) = pick_hex_field::<u16>(&split, 4)? {
                Some(wk & 0x3ff)
            } else {
                None
            }
        },
        health_bits: pick_hex_field(&split, 5)?,
        eccentricity: pick_hex_field(&split, 6)?,
        reference_time: pick_hex_field(&split, 7)?,
        sigma: pick_hex_field(&split, 8)?,
        omega_dot: pick_hex_field(&split, 9)?,
        root_a: pick_hex_field(&split, 10)?,
        omega: pick_hex_field(&split, 11)?,
        omega_o: pick_hex_field(&split, 12)?,
        mo: pick_hex_field(&split, 13)?,
        af0: pick_hex_field(&split, 14)?,
        af1: pick_hex_field(&split, 15)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpalm() {
        match NmeaParser::new().parse_sentence(
            "$GPALM,31,1,02,1617,00,50F6,0F,FD98,FD39,A10CF3,81389B,423632,BD913C,148,001*",
        ) {
            Ok(ps) => match ps {
                ParsedMessage::Alm(alm) => {
                    assert_eq!(alm.source, NavigationSystem::Gps);
                    assert_eq!(alm.prn, Some(2));
                    assert_eq!(alm.week_number, Some(535));
                    assert_eq!(alm.health_bits, Some(0));
                    assert_eq!(alm.eccentricity, Some(0x50F6));
                    assert_eq!(alm.reference_time, Some(0x0f));
                    assert_eq!(alm.sigma, Some(0xfd98));
                    assert_eq!(alm.omega_dot, Some(0xfd39));
                    assert_eq!(alm.root_a, Some(0xa10cf3));
                    assert_eq!(alm.omega, Some(0x81389b));
                    assert_eq!(alm.omega_o, Some(0x423632));
                    assert_eq!(alm.mo, Some(0xbd913c));
                    assert_eq!(alm.af0, Some(0x148));
                    assert_eq!(alm.af1, Some(0x001));
                }
                ParsedMessage::Incomplete => {
                    assert!(false);
                }
                _ => {
                    assert!(false);
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
}
