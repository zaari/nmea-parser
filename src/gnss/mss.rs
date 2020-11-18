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

/// MSS - Multiple Data ID
#[derive(Clone, Debug, PartialEq)]
pub struct MssData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Signal strength (dB)
    pub ss: Option<u8>,

    /// Signal-to-noise ratio
    pub snr: Option<u8>,

    /// Beacon frequency
    pub frequency: Option<f64>,

    /// Beacon bit rate
    pub bit_rate: Option<u32>,

    /// Channel number
    pub channel: Option<u32>,
}

// -------------------------------------------------------------------------------------------------

/// xxMSS: Multiple Data ID
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Mss(MssData {
        source: nav_system,
        ss: pick_number_field(&split, 1)?,
        snr: pick_number_field(&split, 2)?,
        frequency: pick_number_field(&split, 3)?,
        bit_rate: pick_number_field(&split, 4)?,
        channel: pick_number_field(&split, 5)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpmss() {
        match NmeaParser::new().parse_sentence("$GPMSS,55,27,318.0,100,1*57") {
            Ok(ps) => match ps {
                ParsedMessage::Mss(mss) => {
                    assert_eq!(mss.source, NavigationSystem::Gps);
                    assert_eq!(mss.ss, Some(55));
                    assert_eq!(mss.snr, Some(27));
                    assert_eq!(mss.frequency, Some(318.0));
                    assert_eq!(mss.bit_rate, Some(100));
                    assert_eq!(mss.channel, Some(1));
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
