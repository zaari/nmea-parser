use super::*;


#[derive(Clone, Debug, PartialEq)]
pub struct DbsData {
    /// Water depth below surface, meters
    pub depth_meters: Option<f64>,

    /// Water depth below surface, feet
    pub depth_feet: Option<f64>,

    /// Water depth below surface, Fathoms
    pub depth_fathoms: Option<f64>,
}

// -------------------------------------------------------------------------------------------------

/// xxDPT: Depth Below Surface
pub(crate) fn handle(
    sentence: &str,
) -> Result<ParsedMessage, ParseError> {
    let split: Vec<&str> = sentence.split(',').collect();

    Ok(ParsedMessage::Dbs(DbsData{
        depth_meters: pick_number_field(&split, 3)?,
        depth_feet: pick_number_field(&split, 1)?,
        depth_fathoms: pick_number_field(&split, 5)?,
    }))
}


// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    use crate::NmeaParser;

    #[test]
    fn test_parse_dpt() {
        match NmeaParser::new().parse_sentence(
            "$SDDBS,16.9,f,5.2,M,2.8,F*32"
        ) {
            Ok (ps) => match ps {
                ParsedMessage::Dbs(dbs) => {
                    assert_eq!(dbs.depth_meters, Some(5.2));
                    assert_eq!(dbs.depth_feet, Some(16.9));
                    assert_eq!(dbs.depth_fathoms, Some(2.8))
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