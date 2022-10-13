/*
Copyright 2021 Linus Eing

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

/// MTW - Mean Temperature of Water
#[derive(Clone, Debug, PartialEq)]
pub struct MtwData {
    /// Water temperature in degrees Celsius
    pub temperature: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxMTW: Mean Temperature of Water
pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Mtw(MtwData {
        temperature: pick_number_field(&split, 1)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use crate::NmeaParser;

    #[test]
    fn test_parse_dpt() {
        match NmeaParser::new().parse_sentence("$INMTW,17.9,C*1B") {
            Ok(ps) => match ps {
                ParsedMessage::Mtw(mtw) => {
                    assert_eq!(mtw.temperature, Some(17.9))
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
