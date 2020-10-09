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
/// AIVDM type 5: Ship static voyage related data
pub fn handle(bv: &BitVec, _station: Station, own_vessel: bool) -> Result<ParsedSentence, ParseError> {
    return Ok(ParsedSentence::VesselStaticData(VesselStaticData{
        own_vessel:              own_vessel,
        ais_type:                AisClass::ClassB,
        mmsi:                    pick_u64(&bv, 8, 30) as u32,
        ais_version_indicator:   pick_u64(&bv, 38, 2) as u8,
        imo_number: {
            let raw = pick_u64(&bv, 40, 30) as u32;
            match raw { 0 => { None }, _ => { Some(raw) } }
        },
        call_sign: {
            let raw = pick_string(&bv, 70, 7);
            match raw.as_str() { "" => { None }, _ => { Some(raw) }, }
        },
        name: {
            let raw = pick_string(&bv, 112, 20);
            match raw.as_str() { "" => { None }, _ => { Some(raw) }, }
        },
        ship_type: {
            ShipType::new(pick_u64(&bv, 232, 8) as u8)
        },
        cargo_type: {
            CargoType::new(pick_u64(&bv, 232, 8) as u8)
        },
        equipment_vendor_id: { 
            None // part of AIS class B
        },
        equipment_model: { 
            None // part of AIS class B
        },
        equipment_serial_number: {
            None // part of AIS class B
        },
        dimension_to_bow: {
            Some(pick_u64(&bv, 240, 9) as u16)
        },
        dimension_to_stern: {
            Some(pick_u64(&bv, 249, 9) as u16)
        },
        dimension_to_port: {
            Some(pick_u64(&bv, 258, 6) as u16)
        },
        dimension_to_starboard: { 
            Some(pick_u64(&bv, 264, 6) as u16)
        },
        position_fix_type: {
            let raw = pick_u64(&bv, 270, 4) as u8;
            match raw {
                0 => { None },
                _ => { Some(PositionFixType::new(raw)) },
            }
        },
        eta:                     pick_eta(&bv, 274),
        draught10:               Some(pick_u64(&bv, 294, 8) as u8),
        destination: {
            let raw = pick_string(&bv, 302, 20);
            match raw.as_str() { "" => { None }, _ => { Some(raw) }, }
        },
        mothership_mmsi: {
            None
        }
    }));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type5() {
        let mut store = NmeaStore::new();
    
        let s1 = "!AIVDM,2,1,1,A,55?MbV02;H;s<HtKR20EHE:0@T4@Dn2222222216L961O5Gf0NSQEp6ClRp8,0*1C";
        let s2 = "!AIVDM,2,2,1,A,88888888880,2*25";
        
        // Process fragment 1
        match parse_sentence(s1, &mut store) {
            Ok(_) => {
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
        assert_eq!(store.strings_count(), 1);
        
        // Process fragment 2
        match parse_sentence(s2, &mut store) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselStaticData(vsd) => {
                        assert_eq!(vsd.mmsi, 351759000);
                        assert_eq!(vsd.ais_version_indicator, 0);
                        assert_eq!(vsd.imo_number, Some(9134270));
                        assert_eq!(vsd.call_sign, Some("3FOF8".into()));
                        assert_eq!({
                            let callsign = vsd.call_sign.clone().unwrap_or("".into());
                            callsign.len() 
                        }, 5);
                        assert_eq!(vsd.name, Some("EVER DIADEM".into()));
                        assert_eq!(vsd.ship_type, ShipType::Cargo);
                        assert_eq!(vsd.cargo_type, CargoType::Undefined);
                        assert_eq!(vsd.dimension_to_bow, Some(225));
                        assert_eq!(vsd.dimension_to_stern, Some(70));
                        assert_eq!(vsd.dimension_to_port, Some(1));
                        assert_eq!(vsd.dimension_to_starboard, Some(31));
                        assert_eq!(vsd.position_fix_type, Some(PositionFixType::GPS));
                        assert_eq!(vsd.eta, {
                            if let Some(dt) = vsd.eta {
                                Some(Utc.ymd(dt.year(), 5, 15).and_hms(14, 0, 30))
                            } else {
                                None
                            }
                        });
                        assert_eq!(vsd.draught10, Some(122));
                        assert_eq!(vsd.destination, Some("NEW YORK".into()));
                    },
                    ParsedSentence::Incomplete => {
                        assert!(false);
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
        
        // Process fragment 2 (reversed order)
        match parse_sentence(s2, &mut store) {
            Ok(_) => {
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
        assert_eq!(store.strings_count(), 1);
        
        // Process fragment 1 (reversed order)
        match parse_sentence(s1, &mut store) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselStaticData(vsd) => {
                        assert_eq!(vsd.mmsi, 351759000);
                        assert_eq!(vsd.ais_version_indicator, 0);
                        assert_eq!(vsd.imo_number, Some(9134270));
                        assert_eq!(vsd.call_sign, Some("3FOF8".into()));
                        assert_eq!({
                            let callsign = vsd.call_sign.clone().unwrap_or("".into());
                            callsign.len() 
                        }, 5);
                        assert_eq!(vsd.name, Some("EVER DIADEM".into()));
                        assert_eq!(vsd.ship_type, ShipType::Cargo);
                        assert_eq!(vsd.cargo_type, CargoType::Undefined);
                        assert_eq!(vsd.dimension_to_bow, Some(225));
                        assert_eq!(vsd.dimension_to_stern, Some(70));
                        assert_eq!(vsd.dimension_to_port, Some(1));
                        assert_eq!(vsd.dimension_to_starboard, Some(31));
                        assert_eq!(vsd.position_fix_type, Some(PositionFixType::GPS));
                        assert_eq!(vsd.eta, {
                            if let Some(dt) = vsd.eta {
                                let year = dt.naive_utc().year();
                                Some(DateTime::<Utc>::from_utc(
                                     NaiveDate::from_ymd(year, 5, 15).and_hms(14, 0, 30), Utc))
                            } else {
                                None
                            }
                        });
                        assert_eq!(vsd.draught10, Some(122));
                        assert_eq!(vsd.destination, Some("NEW YORK".into()));
                    },
                    ParsedSentence::Incomplete => {
                        assert!(false);
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK");
            }
        }
        
    }
}

