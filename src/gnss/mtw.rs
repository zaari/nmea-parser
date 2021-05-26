use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MtwData {
    /// Water temperature in degrees Celsius
    temperature: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxMTW: Mean Temperature of Water
pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Mtw(MtwData {
        temperature: pick_number_field(&split, 1)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use crate::NmeaParser;

    #[test]
    fn test_parse_dpt() {
        match NmeaParser::new().parse_sentence("$INMTW,17.9,C*1B") {
            Ok(ps) => match ps {
                ParsedMessage::Mtw(mtw) => {
                    assert_eq!(mtw.temperature, Some(17.9))
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
