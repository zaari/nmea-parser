# NMEA Parser for Rust

[![NMEA Parser on crates.io][cratesio-image]][cratesio]
[![NMEA Parser on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/nmea-parser.svg
[cratesio]: https://crates.io/crates/nmea-parser
[docsrs-image]: https://docs.rs/nmea-parser/badge.svg
[docsrs]: https://docs.rs/nmea-parser

This Rust crate aims to cover the most important [AIS] and [GNSS] sentences. It supports both 
class A and B types of AIS.

## Usage

Include the following fragment in your `Cargo.toml` file:

```toml
[dependencies]
nmea-parser = "0.3.1"
```

The following sample program uses the crate to parse the given NMEA sentence and to print some 
fields of the resulting data object:

```rust
use nmea_parser::*;

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
            println!("Heading: {}°",     vdd.heading_true.unwrap());
            println!("");
        },
        ParsedSentence::VesselStaticData(vds) => {
            println!("MMSI:  {}", vds.mmsi);
            println!("Flag:  {}", vds.country().unwrap());
            println!("Name:  {}", vds.name.unwrap());
            println!("Type:  {}", vds.ship_type);
            println!("");
        },
        ParsedSentence::Gga(gga) => {
            println!("Source:    {}",     gga.source);
            println!("Latitude:  {:.3}°", gga.latitude.unwrap());
            println!("Longitude: {:.3}°", gga.longitude.unwrap());
            println!("");
        },
        ParsedSentence::Rmc(rmc) => {
            println!("Source:  {}",        rmc.source);
            println!("Speed:   {:.1} kts", rmc.sog_knots.unwrap());
            println!("Bearing: {}°",       rmc.bearing.unwrap());
            println!("Time:    {}",        rmc.timestamp.unwrap());
            println!("");
        },
        _ => {
        }
    }
}
```

The program outputs the following lines:

```
MMSI:  271041815
Flag:  TR
Name:  PROGUY
Type:  passenger

Source:    Galileo
Latitude:  48.117°
Longitude: 11.517°

```

## Features

The following features are included in the published version of the crate.

|Feature          |Description                                                |
|-----------------|-----------------------------------------------------------|
|AIS sentences    |VDM/VDO types 1-3, 5, 18-19 and 24                         |
|GNSS sentences   |GGA, RMC, GSA, GSV, VTG, GLL                               |
|Satellite systems|GPS, GLONASS, Galileo, BeiDou, NavIC and QZSS              | 

## Roadmap

Until version 1.0 refactoring and renaming of crate's code elements is likely to happen but the goal
is to the make breaking changes as early as possible. The following table shows the plan to include
different sentences in the crate. Prioritisation is based on estimated significance and 
implementation effort of each of them.

|Version |Category    |Goal                                                   |
|--------|------------|-------------------------------------------------------|
|0.4     |AIS         |VDM/VDO types 4, 9-17, 21, 27                          |
|0.5     |GNSS        |ALM, TRF, STN, VBW, XTC, XTE, ZDA                      |
|0.6     |AIS         |VDM/VDO types 20, 22, 23, 25, 26                       |
|0.7     |AIS         |VDM/VDO types 6-8                                      |
|1.0     |meta        |Stable API, optimizations, enhanced documentation      |
|1.2     |GNSS        |AAM, BOD, BWC, R00, RMB, RTE, WPL, ZTG                 |
|1.3     |GNSS        |APB, DTM, RMA, GRS, GST, MSK, MSS, STN, VBW            |

[AIS]: https://en.wikipedia.org/wiki/Automatic_identification_system
[GNSS]: https://en.wikipedia.org/wiki/Satellite_navigation

## License

The crate is licensed under [Apache 2.0 license] which also includes liability and warranty
statements.

[Apache 2.0 license]: LICENSE
