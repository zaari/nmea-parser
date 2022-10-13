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

/// MWV - Wind speed and angle
#[derive(Clone, Debug, PartialEq)]
pub struct MwvData {
    /// wind angle, 0 to 359 degrees
    pub wind_angle: Option<f64>,

    /// Reference, True/Relative (true = relative, false = true, None = unknown)
    pub relative: Option<bool>,

    /// Wind speed - knots
    pub wind_speed_knots: Option<f64>,

    /// Wind speed - km/h
    pub wind_speed_kmh: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxMWV: Wind speed and angle

pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Mwv(MwvData {
        wind_angle: pick_number_field(&split, 1)?,
        relative: match pick_string_field(&split, 2).unwrap().as_str() {
            "R" => Some(true),
            "T" => Some(false),
            _ => None,
        },
        wind_speed_knots: match pick_string_field(&split, 4).unwrap().as_str() {
            "N" => pick_number_field(&split, 3)?,
            "M" => Some(pick_number_field::<f64>(&split, 3)?.unwrap() * 1.943844),
            "K" => Some(pick_number_field::<f64>(&split, 3)?.unwrap() * 0.539957),
            _ => None,
        },
        wind_speed_kmh: match pick_string_field(&split, 4).unwrap().as_str() {
            "N" => Some(pick_number_field::<f64>(&split, 3)?.unwrap() * 1.852),
            "M" => Some(pick_number_field::<f64>(&split, 3)?.unwrap() * 3.6),
            "K" => pick_number_field(&split, 3)?,
            _ => None,
        },
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mwv() {
        match NmeaParser::new().parse_sentence("$WIMWV,295.4,T,33.3,N,A*1C") {
            Ok(ps) => match ps {
                ParsedMessage::Mwv(mwv) => {
                    assert_eq!(mwv.wind_angle, Some(295.4));
                    assert_eq!(mwv.relative, Some(false));
                    assert_eq!(mwv.wind_speed_knots, Some(33.3));
                    assert_eq!(mwv.wind_speed_kmh, Some(33.3 * 1.852));
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
