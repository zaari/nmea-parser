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
/// AIVDM type 19: Extended Class B Equipment Position Report
pub fn handle(_bv: &BitVec, _station: Station, _own_vessel: bool) -> Result<ParsedSentence, String> {
    // TODO: implementation (Class B)
    return Err("Unsupported AIVDM message type: 19".into());
}

#[cfg(test)]
mod test {
//    use super::*;

//    #[test]
//    fn test_parse_avidm_type19() {
//        assert!(false);
//    }
}
