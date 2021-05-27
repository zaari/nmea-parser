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

/// HDT - Heading, true
#[derive(Clone, Debug, PartialEq)]
pub struct HdtData {
    /// Heading - true
    heading_true: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxHDT: Heading, true

pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Hdt(HdtData {
        heading_true: pick_number_field(&split, 1)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hdt() {
        match NmeaParser::new().parse_sentence("$IIHDT,15.0,T*16") {
            Ok(ps) => match ps {
                ParsedMessage::Hdt(hdt) => {
                    assert_eq!(hdt.heading_true, Some(15.0))
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
