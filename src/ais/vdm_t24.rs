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

/// AIS VDM/VDO type 24: Static data report
pub(crate) fn handle(bv: &BitVec, _station: Station, store: &mut NmeaParser, own_vessel: bool)
-> Result<ParsedSentence, ParseError> {
    // Check whether the message bit layout follows part A or part B format
    // We use two complementary booleans to make the code more readable.
    let (part_a, part_b) = match pick_u64(&bv, 38, 2) {
        0 => { (true, false) },
        1 => { (false, true) },
        _ => {
            return Err(format!("AIVDM type 24 part number has unexpected value: {}", 
                               pick_u64(&bv, 38, 2)).into());
        }
    };
    
    // Pick the fields 
    let vsd = VesselStaticData{
        own_vessel:              own_vessel,
        ais_type:                AisClass::ClassB,
        mmsi:                    pick_u64(&bv, 8, 30) as u32,
        ais_version_indicator:   0,
        imo_number:              None,
        call_sign: {
            if part_b {
                let raw = pick_string(&bv, 90, 7);
                match raw.as_str() { "" => { None }, _ => { Some(raw) }, }
            } else {
                None
            }
        },
        name: {
            if part_a {
                let raw = pick_string(&bv, 40, 120);
                match raw.as_str() { "" => { None }, _ => { Some(raw) }, }
            } else {
                None
            }
        },
        ship_type: {
            if part_b {
                ShipType::new(pick_u64(&bv, 40, 8) as u8)
            } else {
                ShipType::NotAvailable
            }
        },
        cargo_type: {
            if part_b {
                CargoType::new(pick_u64(&bv, 40, 8) as u8)
            } else {
                CargoType::Undefined
            }
        },
        equipment_vendor_id: {
            if part_b {
                Some(pick_string(&bv, 48, 3))
            } else {
                None
            }
        },
        equipment_model: {
            if part_b {
                Some(pick_u64(&bv, 66, 4) as u8)
            } else {
                None
            }
        },
        equipment_serial_number: {
            if part_b {
                Some(pick_u64(&bv, 70, 20) as u32)
            } else {
                None
            }
        },
        dimension_to_bow: {
            if part_b {
                Some(pick_u64(&bv, 132, 9) as u16)
            } else {
                None
            }
        },
        dimension_to_stern: {
            if part_b {
                Some(pick_u64(&bv, 141, 9) as u16)
            } else {
                None
            }
        },
        dimension_to_port: {
            if part_b {
                Some(pick_u64(&bv, 150, 6) as u16)
            } else {
                None
            }
        },
        dimension_to_starboard: { 
            if part_b {
                Some(pick_u64(&bv, 156, 6) as u16)
            } else {
                None
            }
        },
        position_fix_type:  None,
        eta:                None,
        draught10:          None,
        destination:        None,
        mothership_mmsi: {
            if part_b {
                Some(pick_u64(&bv, 132, 30) as u32)
            } else {
                None
            }
        }
    };

    // Check whether we can return a complete or incomplete response
    if let Some(vsd2) = store.pull_vsd(vsd.mmsi) {
        Ok(ParsedSentence::VesselStaticData(vsd.merge(&vsd2)?))
    } else {
        store.push_vsd(vsd.mmsi, vsd);
        Ok(ParsedSentence::Incomplete)
    }
}

impl VesselStaticData {
    /// Merge two data structures together. This is used to combine part A and B
    /// of class B AIVDM type 24 messages.
    fn merge(&self, other: &VesselStaticData) -> Result<VesselStaticData, String> {
        if self.ais_type != other.ais_type {
            Err(format!("Mismatching AIS types: {} != {}", self.ais_type, other.ais_type))
        } else if self.mmsi != other.mmsi {
            Err(format!("Mismatching MMSI numbers: {} != {}", self.mmsi, other.mmsi))
        } else if self.imo_number != other.imo_number {
            Err(format!("Mismatching MMSI numbers: {} != {}", self.mmsi, other.mmsi))
        } else if self.ais_version_indicator != other.ais_version_indicator {
            Err(format!("Mismatching AIS version indicators: {} != {}", 
                        self.ais_version_indicator, other.ais_version_indicator))
        } else {
            Ok(VesselStaticData{
                own_vessel: self.own_vessel,
                ais_type: self.ais_type.clone(),
                mmsi: self.mmsi.clone(),
                ais_version_indicator: self.ais_version_indicator.clone(),  
                imo_number: choose_some(self.imo_number, other.imo_number),
                call_sign: choose_some_string(&self.call_sign, &other.call_sign),
                name: choose_some_string(&self.name, &other.name),
                ship_type: {
                    if self.ship_type != ShipType::NotAvailable {
                        self.ship_type
                    } else {
                        other.ship_type
                    }
                },
                cargo_type: {
                    if self.cargo_type != CargoType::Undefined {
                        self.cargo_type
                    } else {
                        other.cargo_type
                    }
                },
                equipment_vendor_id: choose_some_string(&self.equipment_vendor_id, 
                                                        &other.equipment_vendor_id),
                equipment_model: choose_some(self.equipment_model, other.equipment_model),
                equipment_serial_number: choose_some(self.equipment_serial_number, 
                                                     other.equipment_serial_number),
                dimension_to_bow: choose_some(self.dimension_to_bow, other.dimension_to_bow),
                dimension_to_stern: choose_some(self.dimension_to_stern, other.dimension_to_stern),
                dimension_to_port: choose_some(self.dimension_to_port, other.dimension_to_port),
                dimension_to_starboard: choose_some(self.dimension_to_starboard, 
                                                    other.dimension_to_starboard),
                position_fix_type: choose_some(self.position_fix_type, other.position_fix_type),
                eta: choose_some(self.eta, other.eta),
                draught10: choose_some(self.draught10, other.draught10),
                destination: choose_some_string(&self.destination, &other.destination),
                mothership_mmsi: choose_some(self.mothership_mmsi, other.mothership_mmsi),
            })
        }
    }
}

/// Choose the argument which is Some. If both are Some, choose the first one.
fn choose_some<T>(a: Option<T>, b: Option<T>) -> Option<T> {
    if a.is_some() {
        a
    } else {
        b
    }
}

/// Choose the argument which is Some. If both are Some, choose the first one.
fn choose_some_string(a: &Option<String>, b: &Option<String>) -> Option<String> {
    if a.is_some() {
        a.clone()
    } else {
        b.clone()
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_vdm_type24() {
        let mut p = NmeaParser::new();
    
        let s1 = "!AIVDM,1,1,,A,H42O55i18tMET00000000000000,2*6D";
        match p.parse_sentence(s1) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselStaticData(_) => {
                        assert!(false); return;
                    },
                    ParsedSentence::Incomplete => {
                        // As expected
                    },
                    _ => {
                        assert!(false); return;
                    }
                }
            },
            Err(e) => {
                assert_eq!(e.to_string(), "OK"); return;
            }
        }
        let s2 = "!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40";
        match p.parse_sentence(s2) {
            Ok(ps) => {
                match ps {
                    // The expected result
                    ParsedSentence::VesselStaticData(vsd) => {
                        assert_eq!(vsd.mmsi, 271041815);
                        assert_eq!(vsd.ais_version_indicator, 0);
                        assert_eq!(vsd.imo_number, None);
                        assert_eq!(vsd.call_sign, Some("TC6163".into()));
                        assert_eq!(vsd.name, Some("PROGUY".into()));
                        assert_eq!(vsd.ship_type, ShipType::Passenger);
                        assert_eq!(vsd.cargo_type, CargoType::Undefined);

                        assert_eq!(vsd.equipment_vendor_id, Some("1D0".into()));
//                                assert_eq!(vsd.equipment_model, None);
//                                assert_eq!(vsd.equipment_serial_number, None);
//                                assert_eq!(vsd.mothership_mmsi, None);
// TODO: find the right hand side of the variables above
                        
                        assert_eq!(vsd.dimension_to_bow, Some(0));
                        assert_eq!(vsd.dimension_to_stern, Some(15));
                        assert_eq!(vsd.dimension_to_port, Some(0));
                        assert_eq!(vsd.dimension_to_starboard, Some(5));
                        
                        assert_eq!(vsd.position_fix_type, None);
                        assert_eq!(vsd.eta, None);
                        assert_eq!(vsd.draught10, None);
                        assert_eq!(vsd.destination, None);
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

