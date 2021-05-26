use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct DptData {
    /// Water depth relative to transducer, meters
    pub depth_relative_to_transducer: Option<f64>,

    /// Offset from transducer, meters positive means distance from transducer to water line negative means distance from transducer to keel
    pub transducer_offset: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxDPT: Depth of Water
pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Dpt(DptData {
        depth_relative_to_transducer: pick_number_field(&split, 1)?,
        transducer_offset: pick_number_field(&split, 2)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use crate::NmeaParser;

    #[test]
    fn test_parse_dpt() {
        match NmeaParser::new().parse_sentence("$SDDPT,17.5,0.3*67") {
            Ok(ps) => match ps {
                ParsedMessage::Dpt(dpt) => {
                    assert_eq!(dpt.depth_relative_to_transducer, Some(17.5));
                    assert_eq!(dpt.transducer_offset, Some(0.3));
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
