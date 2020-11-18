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

/// STN - MSK Receiver Signal
#[derive(Clone, Debug, PartialEq)]
pub struct StnData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Talker id numer (0-99)
    pub talker_id: Option<u8>,
}

// -------------------------------------------------------------------------------------------------

/// xxSTN: MSK Receiver Signal
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Stn(StnData {
        source: nav_system,
        talker_id: pick_number_field(&split, 1)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpstn() {
        match NmeaParser::new().parse_sentence("$GPSTN,23") {
            Ok(ps) => match ps {
                ParsedMessage::Stn(stn) => {
                    assert_eq!(stn.source, NavigationSystem::Gps);
                    assert_eq!(stn.talker_id, Some(23));
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
