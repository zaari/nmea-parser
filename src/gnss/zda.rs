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

/// ZDA - Time and date
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ZdaData {
    /// Navigation system
    pub source: NavigationSystem,

    /// UTC
    #[serde(with = "json_date_time_utc")]
    pub timestamp_utc: Option<DateTime<Utc>>,

    /// Local time zone offset
    #[serde(with = "json_fixed_offset")]
    pub timezone_local: Option<FixedOffset>,
}

// -------------------------------------------------------------------------------------------------

/// xxZDA: MSK Receiver Signal
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Zda(ZdaData {
        source: nav_system,
        timestamp_utc: parse_hhmmss_ss(
            split.get(1).unwrap_or(&""),
            pick_date_with_fields(&split, 4, 3, 2, 0, 0, 0, 0)?,
        )
        .ok(),
        timezone_local: pick_timezone_with_fields(&split, 5, 6).ok(),
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cpzda() {
        match NmeaParser::new().parse_sentence("$GPZDA,072914.00,31,05,2018,-03,00") {
            Ok(ps) => match ps {
                ParsedMessage::Zda(zda) => {
                    assert_eq!(zda.source, NavigationSystem::Gps);
                    assert_eq!(
                        zda.timestamp_utc,
                        Some(Utc.ymd(2018, 5, 31).and_hms(7, 29, 14))
                    );
                    assert_eq!(
                        zda.timezone_local,
                        Some(FixedOffset::east(-3 * 3600 - 0 * 60))
                    );
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
