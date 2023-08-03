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

/// DTM - Datum being used
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DtmData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Local datum code
    pub datum_id: Option<String>,

    /// Local datum subdivision code
    pub datum_sub_id: Option<String>,

    /// Latitude offset in degrees
    pub lat_offset: Option<f64>,

    /// Longitude offset in degrees
    pub lon_offset: Option<f64>,

    /// Altitude offset in metres
    pub alt_offset: Option<f64>,

    /// Reference datum code
    pub ref_datum_id: Option<String>,
}

// -------------------------------------------------------------------------------------------------

/// xxDTM: Datum being used
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Dtm(DtmData {
        source: nav_system,
        datum_id: pick_string_field(&split, 1),
        datum_sub_id: pick_string_field(&split, 2),
        lat_offset: parse_latitude_m_m(split.get(3).unwrap_or(&""), split.get(4).unwrap_or(&""))?,
        lon_offset: parse_longitude_m_m(split.get(5).unwrap_or(&""), split.get(6).unwrap_or(&""))?,
        alt_offset: pick_number_field(&split, 7)?,
        ref_datum_id: pick_string_field(&split, 8),
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpdtm() {
        match NmeaParser::new().parse_sentence("$GPDTM,999,,0.002,S,0.005,E,005.8,W84*1A") {
            Ok(ps) => match ps {
                ParsedMessage::Dtm(dtm) => {
                    assert_eq!(dtm.source, NavigationSystem::Gps);
                    assert_eq!(dtm.datum_id, Some("999".into()));
                    assert_eq!(dtm.datum_sub_id, None);
                    assert::close(dtm.lat_offset.unwrap_or(0.0), -0.000033, 0.000001);
                    assert::close(dtm.lon_offset.unwrap_or(0.0), 0.000083, 0.000001);
                    assert_eq!(dtm.alt_offset, Some(5.8));
                    assert_eq!(dtm.ref_datum_id, Some("W84".into()));
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
