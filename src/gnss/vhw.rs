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

/// VHW - Water speed and heading
#[derive(Clone, Debug, PartialEq)]
pub struct VhwData {
    /// Heading - true
    heading_true: Option<f64>,

    /// Heading - magnetic
    heading_magnetic: Option<f64>,

    /// Velocity relative to water - knots
    speed_through_water_knots: Option<f64>,

    /// Velocity relative to water - km/h
    speed_through_water_kmh: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

// xxVHW: Water speed and heading

pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Vhw(VhwData {
        heading_true: pick_number_field(&split, 1)?,
        heading_magnetic: pick_number_field(&split, 3)?,
        speed_through_water_knots: pick_number_field(&split, 5)?,
        speed_through_water_kmh: pick_number_field(&split, 7)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vhw() {
        match NmeaParser::new().parse_sentence("$IIVHW,15.0,T,15.0,M,6.3,N,11.8,K*68") {
            Ok(ps) => match ps {
                ParsedMessage::Vhw(vhw) => {
                    assert_eq!(vhw.heading_true, Some(15.0));
                    assert_eq!(vhw.heading_magnetic, Some(15.0));
                    assert_eq!(vhw.speed_through_water_knots, Some(6.3));
                    assert_eq!(vhw.speed_through_water_kmh, Some(11.8));
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
