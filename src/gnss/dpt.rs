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

/// DPT - Depth of Water
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DptData {
    /// Water depth relative to transducer, meters
    pub depth_relative_to_transducer: Option<f64>,

    /// Offset from transducer, meters positive means distance from transducer to water line negative means distance from transducer to keel
    pub transducer_offset: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxDPT: Depth of Water
pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Dpt(DptData {
        depth_relative_to_transducer: pick_number_field(&split, 1)?,
        transducer_offset: pick_number_field(&split, 2)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use crate::NmeaParser;

    #[test]
    fn test_parse_dpt() {
        match NmeaParser::new().parse_sentence("$SDDPT,17.5,0.3*67") {
            Ok(ps) => match ps {
                ParsedMessage::Dpt(dpt) => {
                    assert_eq!(dpt.depth_relative_to_transducer, Some(17.5));
                    assert_eq!(dpt.transducer_offset, Some(0.3));
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
