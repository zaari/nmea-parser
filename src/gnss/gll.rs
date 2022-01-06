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

/// GLL - geographic Position - Latitude/Longitude
#[derive(Clone, Debug, PartialEq)]
pub struct GllData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Latitude in degrees.
    pub latitude: Option<f64>,

    /// Longitude in degrees.
    pub longitude: Option<f64>,

    /// UTC of position fix
    pub timestamp: Option<DateTime<Utc>>,

    /// True = data valid, false = data invalid.
    pub data_valid: Option<bool>,

    /// FAA mode indicator (NMEA 2.3 and later).
    pub faa_mode: Option<FaaMode>,
}

impl LatLon for GllData {
    fn latitude(&self) -> Option<f64> {
        self.latitude
    }

    fn longitude(&self) -> Option<f64> {
        self.longitude
    }
}

// -------------------------------------------------------------------------------------------------

/// xxGLL: Geographic Position, Latitude / Longitude and time.
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let now: DateTime<Utc> = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0);
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Gll(GllData {
        source: nav_system,
        latitude: parse_latitude_ddmm_mmm(
            split.get(1).unwrap_or(&""),
            split.get(2).unwrap_or(&""),
        )?,
        longitude: parse_longitude_dddmm_mmm(
            split.get(3).unwrap_or(&""),
            split.get(4).unwrap_or(&""),
        )?,
        timestamp: parse_hhmmss(split.get(5).unwrap_or(&""), now).ok(),
        data_valid: {
            match *split.get(6).unwrap_or(&"") {
                "A" => Some(true),
                "V" => Some(false),
                _ => None,
            }
        },
        faa_mode: FaaMode::new(split.get(7).unwrap_or(&"")).ok(),
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_gagll() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("$GAGLL,4916.45,N,12311.12,W,225444,A,D*48") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Gll(gll) => {
                        assert_eq!(gll.source, NavigationSystem::Galileo);
                        assert::close(gll.latitude.unwrap_or(0.0), 49.3, 0.1);
                        assert::close(gll.longitude.unwrap_or(0.0), -123.2, 0.1);
                        assert_eq!(gll.timestamp, {
                            Some(Utc.ymd(2000, 01, 01).and_hms(22, 54, 44))
                        });
                        assert_eq!(gll.data_valid, Some(true));
                        assert_eq!(gll.faa_mode, Some(FaaMode::Differential));
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
    }
}
