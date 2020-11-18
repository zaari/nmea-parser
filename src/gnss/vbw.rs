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

/// VBW - Dual Ground/Water Speed
#[derive(Clone, Debug, PartialEq)]
pub struct VbwData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Longitudinal water speed, knots     
    pub lon_water_speed_knots: Option<f64>,

    /// Transverse water speed, knots
    pub tr_water_speed_knots: Option<f64>,

    /// Water speed status
    pub water_speed_valid: Option<bool>,

    /// Longitudinal ground speed, knots     
    pub lon_ground_speed_knots: Option<f64>,

    /// Transverse ground speed, knots
    pub tr_ground_speed_knots: Option<f64>,

    /// Ground speed status
    pub ground_speed_valid: Option<bool>,
}

// -------------------------------------------------------------------------------------------------

/// xxVBW: Dual Ground/Water Speed
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Vbw(VbwData {
        source: nav_system,
        lon_water_speed_knots: pick_number_field(&split, 1)?,
        tr_water_speed_knots: pick_number_field(&split, 2)?,
        water_speed_valid: {
            match *split.get(3).unwrap_or(&"") {
                "A" => Some(true),
                "" => None,
                _ => Some(false),
            }
        },
        lon_ground_speed_knots: pick_number_field(&split, 4)?,
        tr_ground_speed_knots: pick_number_field(&split, 5)?,
        ground_speed_valid: {
            match *split.get(6).unwrap_or(&"") {
                "A" => Some(true),
                "" => None,
                _ => Some(false),
            }
        },
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpvbw() {
        match NmeaParser::new().parse_sentence("$GPVBW,2.0,1.5,A,2.1,1.6,X") {
            Ok(ps) => match ps {
                ParsedMessage::Vbw(vbw) => {
                    assert_eq!(vbw.source, NavigationSystem::Gps);
                    assert::close(vbw.lon_water_speed_knots.unwrap_or(0.0), 2.0, 0.1);
                    assert::close(vbw.tr_water_speed_knots.unwrap_or(0.0), 1.5, 0.1);
                    assert_eq!(vbw.water_speed_valid, Some(true));
                    assert::close(vbw.lon_ground_speed_knots.unwrap_or(0.0), 2.1, 0.1);
                    assert::close(vbw.tr_ground_speed_knots.unwrap_or(0.0), 1.6, 0.1);
                    assert_eq!(vbw.ground_speed_valid, Some(false));
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
