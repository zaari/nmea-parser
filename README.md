# NMEA Parser for Rust

[![NMEA Parser on crates.io][cratesio-image]][cratesio]
[![NMEA Parser on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/nmea-parser.svg
[cratesio]: https://crates.io/crates/nmea-parser
[docsrs-image]: https://docs.rs/nmea-parser/badge.svg
[docsrs]: https://docs.rs/nmea-parser

This Rust crate aims to cover the most important [AIS] and [GNSS] sentences. It 
supports both class A and B types of AIS.

## Usage

Include the following fragment in your `Cargo.toml` file:

```toml
[dependencies]
nmea-parser = "0.2.0"
```

The following sample program uses the crate to parse the given NMEA sentence 
and to print some  fields of the resulting data object:

```rust
use nmea_parser::*;

pub fn main() -> Result<(), String> {
    let mut store = NmeaStore::new();
    let sentences = vec![
      "!AIVDM,1,1,,A,H42O55i18tMET00000000000000,2*6D",
      "!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40",
      "$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*56",
    ];

    for sentence in sentences {    
        match nmea_parser::parse_sentence(sentence, &mut store)? {
            ParsedSentence::VesselDynamicData(vdd) => {
                println!("MMSI:  {}",        vdd.mmsi);
                println!("Speed: {:.1} kts", vdd.sog_knots.unwrap());
            },
            ParsedSentence::VesselStaticData(vds) => {
                println!("MMSI:  {}", vds.mmsi);
                println!("Flag:  {}", vds.country().unwrap());
                println!("Name:  {}", vds.name.unwrap());
                println!("Type:  {}", vds.ship_type);
            },
            ParsedSentence::Gga(gga) => {
                println!("System:    {}",     gga.system);
                println!("Latitude:  {:.3}°", gga.latitude.unwrap());
                println!("Longitude: {:.3}°", gga.longitude.unwrap());
            },
            ParsedSentence::Rmc(rmc) => {
                println!("System:  {}",        rmc.system);
                println!("Speed:   {:.1} kts", rmc.speed_knots.unwrap());
                println!("Bearing: {}°",       rmc.bearing.unwrap());
                println!("Time:    {}",        rmc.timestamp.unwrap());
            },
            _ => {
            }
        }
    }
    Ok(())
}
```

The program should output the following lines:

```
MMSI:  271041815
Flag:  TR
Name:  PROGUY
Type:  passenger
System:    Galileo
Latitude:  48.117°
Longitude: 11.517°
```

## Features

The following features are included in the current version of the crate.

|Feature          |Description                                                                        |
|-----------------|-----------------------------------------------------------------------------------|
|AIS sentences    |VDM/VDO types 1-3, 5, 18-19 and 24                                                 |
|GNSS sentences   |GGA, RMC, GSA, GSV, VTG, GLL                                                       |
|Satellite systems|GPS, GLONASS, Galileo, BeiDou, NavIC and QZSS                                      | 

## Feature Roadmap

Before version 1.0 refactoring and renaming of crate's code elements is likely to happen.

|Version |Category    |Goal                                                                 |
|--------|------------|---------------------------------------------------------------------|
|0.3     |AIS         |AIS VDM/VDO t4, t6-t17, t20-t23, t25-27                              |
|1.0     |meta        |API freeze, optimizations, enhanced documentation                    |
|1.1     |GNSS        |ALM, TRF, STN, VBW, XTC, XTE, ZDA                                    |
|1.2     |GNSS, route |AAM, BOD, BWC, R00, RMB, RTE, WPL, ZTG                               |
|1.3     |GNSS, misc  |APB, DTM, RMA, GRS, GST, MSK, MSS, STN, VBW                          |

[AIS]: https://en.wikipedia.org/wiki/Automatic_identification_system
[GNSS]: https://en.wikipedia.org/wiki/Satellite_navigation

