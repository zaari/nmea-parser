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
/// GSA - GNSS dilution of position (DOP) and active satellites
#[derive(Clone, Debug, PartialEq)]
pub struct GsaData {
    /// Navigation system
    pub source: NavigationSystem,

    /// Mode 1: true = automatic, false = manual
    pub mode1_automatic: Option<bool>,

    /// Mode 2, fix type:
    pub mode2_3d: Option<GsaFixMode>,

    /// PRN numbers used (space for 12)
    pub prn_numbers: Vec<u8>,

    /// Position (3D) dilution of precision
    pub pdop: Option<f64>,

    /// Horizontal dilution of precision
    pub hdop: Option<f64>,

    /// Vertical dilution of precision
    pub vdop: Option<f64>,
}

/// GSA position fix type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GsaFixMode {
    /// No fix.
    NotAvailable,

    /// 2D fix.
    Fix2D,

    /// 3d fix.
    Fix3D,
}

impl core::fmt::Display for GsaFixMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GsaFixMode::NotAvailable => write!(f, "no available"),
            GsaFixMode::Fix2D => write!(f, "2D fix"),
            GsaFixMode::Fix3D => write!(f, "3D fix"),
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// xxGSA: GPS DOP and active satellites
pub(crate) fn handle(
    sentence: &str,
    nav_system: NavigationSystem,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Gsa(GsaData {
        source: nav_system,
        mode1_automatic: {
            let s = split.get(1).unwrap_or(&"");
            match *s {
                "M" => Some(false),
                "A" => Some(true),
                "" => None,
                _ => {
                    return Err(format!("Invalid GPGSA mode: {}", s).into());
                }
            }
        },
        mode2_3d: {
            let s = split.get(2).unwrap_or(&"");
            match *s {
                "1" => Some(GsaFixMode::NotAvailable),
                "2" => Some(GsaFixMode::Fix2D),
                "3" => Some(GsaFixMode::Fix3D),
                "" => None,
                _ => {
                    return Err(format!("Invalid GPGSA fix type: {}", s).into());
                }
            }
        },
        prn_numbers: {
            let mut v = Vec::with_capacity(12);
            for i in 3..15 {
                if split.get(i).unwrap_or(&"") != &"" {
                    if let Some(val) = pick_number_field(&split, i)? {
                        v.push(val);
                    }
                }
            }
            v
        },
        pdop: pick_number_field(&split, 15)?,
        hdop: pick_number_field(&split, 16)?,
        vdop: pick_number_field(&split, 17)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_gpgsa() {
        let mut p = NmeaParser::new();
        match p.parse_sentence("$GPGSA,A,3,19,28,14,18,27,22,31,39,,,,,1.7,1.0,1.3*34") {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedMessage::Gsa(gsa) => {
                        assert_eq!(gsa.mode1_automatic, Some(true));
                        assert_eq!(gsa.mode2_3d, Some(GsaFixMode::Fix3D));
                        assert_eq!(gsa.prn_numbers, vec![19, 28, 14, 18, 27, 22, 31, 39]);
                        assert_eq!(gsa.pdop, Some(1.7));
                        assert_eq!(gsa.hdop, Some(1.0));
                        assert_eq!(gsa.vdop, Some(1.3));
                    }
                    ParsedMessage::Incomplete => {
                        assert!(false);
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
