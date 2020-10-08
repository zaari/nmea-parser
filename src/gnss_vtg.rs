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

#[doc(hidden)]
/// xxVTG: Track Made Good and Ground Speed
pub fn handle(sentence: &str, nav_system: NavigationSystem, _store: &mut NmeaStore) 
              -> Result<ParsedSentence, String> {
    let split: Vec<&str> = sentence.split(',').collect();

    return Ok(ParsedSentence::Vtg(VelocityMadeGood{
        system: nav_system,
        cog_true:          pick_number_field(&split, 1).ok().unwrap_or(None),
        cog_magnetic:      pick_number_field(&split, 3).ok().unwrap_or(None),
        sog_knots: pick_number_field(&split, 5).ok().unwrap_or(None),
        sog_kph:   pick_number_field(&split, 7).ok().unwrap_or(None),
        faa_mode:               FaaMode::new(split.get(9).unwrap_or(&"")).ok(),
    }));
}    

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_bdvtg() {
        let mut store = NmeaStore::new();
        match parse_sentence("$BDVTG,054.7,T,034.4,M,005.5,N,010.2,K,D*31", &mut store) 
        {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::Vtg(vtg) => {
                        assert_eq!(vtg.system, NavigationSystem::Beidou);
                        assert::close(vtg.cog_true.unwrap_or(0.0), 54.7, 0.1);
                        assert::close(vtg.cog_magnetic.unwrap_or(0.0), 34.4, 0.1);
                        assert::close(vtg.sog_knots.unwrap_or(0.0), 5.5, 0.1);
                        assert::close(vtg.sog_kph.unwrap_or(0.0), 10.2, 0.1);
                        assert_eq!(vtg.faa_mode, Some(FaaMode::Differential));
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            Err(e) => {
                assert_eq!(e, "OK");
            }
        }
    }
}

