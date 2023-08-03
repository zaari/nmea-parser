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

/// DBS - Depth Below Surface
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DbsData {
    /// Water depth below surface, meters
    pub depth_meters: Option<f64>,

    /// Water depth below surface, feet
    pub depth_feet: Option<f64>,

    /// Water depth below surface, Fathoms
    pub depth_fathoms: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxDBS: Depth Below Surface
pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Dbs(DbsData {
        depth_meters: pick_number_field(&split, 3)?,
        depth_feet: pick_number_field(&split, 1)?,
        depth_fathoms: pick_number_field(&split, 5)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use crate::NmeaParser;

    #[test]
    fn test_parse_dpt() {
        match NmeaParser::new().parse_sentence("$SDDBS,16.9,f,5.2,M,2.8,F*32") {
            Ok(ps) => match ps {
                ParsedMessage::Dbs(dbs) => {
                    assert_eq!(dbs.depth_meters, Some(5.2));
                    assert_eq!(dbs.depth_feet, Some(16.9));
                    assert_eq!(dbs.depth_fathoms, Some(2.8))
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
