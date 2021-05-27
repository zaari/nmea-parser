use super::*;

/// HDT - Heading, true
#[derive(Clone, Debug, PartialEq)]
pub struct HdtData {
    /// Heading - true
    heading_true: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxHDT: Heading, true

pub(crate) fn handle(sentence: &str) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Hdt(HdtData {
        heading_true: pick_number_field(&split, 1)?,
    }))
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_hdt() {
        match NmeaParser::new().parse_sentence("$IIHDT,15.0,T*16") {
            Ok(ps) => match ps {
                ParsedMessage::Hdt(hdt) => {
                    assert_eq!(hdt.heading_true, Some(15.0))
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
