# NMEA Parser for Rust

[![NMEA Parser on crates.io][cratesio-image]][cratesio]
[![NMEA Parser on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/nmea-parser.svg
[cratesio]: https://crates.io/crates/nmea-parser
[docsrs-image]: https://docs.rs/nmea-parser/badge.svg
[docsrs]: https://docs.rs/nmea-parser

This [Rust] crate aims to cover all [AIS] sentences and the most important
[GNSS] sentences used with [NMEA 0183] standard. It supports both AIS class A
and class B.

## Usage

Include the following fragment in your `Cargo.toml` file:

```toml
[dependencies]
nmea-parser = "0.4.1"
```

The following sample program uses the crate to parse the given NMEA sentences
and to print some parsed fields. The program relies on `unwrap()` function 
to simplify the example. In real-life applications proper handling of `None` 
cases is needed.

```rust
use nmea_parser::*;

// Create parser and define sample sentences
let mut parser = NmeaParser::new();
let sentences = vec![
  "!AIVDM,1,1,,A,H42O55i18tMET00000000000000,2*6D",
  "!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40",
  "$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*56",
];

// Parse the sentences and print the parsed data 
for sentence in sentences {    
    match parser.parse_sentence(sentence)? {
        ParsedSentence::VesselDynamicData(vdd) => {
            println!("MMSI:    {}",        vdd.mmsi);
            println!("Speed:   {:.1} kts", vdd.sog_knots.unwrap());
            println!("Heading: {}°",       vdd.heading_true.unwrap());
            println!("");
        },
        ParsedSentence::VesselStaticData(vsd) => {
            println!("MMSI:  {}", vsd.mmsi);
            println!("Flag:  {}", vsd.decode_country().unwrap());
            println!("Name:  {}", vsd.name.unwrap());
            println!("Type:  {}", vsd.ship_type);
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
Details about version history can be found from the [changelog].

|Feature          |Description                                                |
|-----------------|-----------------------------------------------------------|
|AIS sentences    |VDM/VDO types 1-5, 18-19, 21, 24 and 27                    |
|GNSS sentences   |GGA, RMC, GSA, GSV, VTG, GLL                               |
|Satellite systems|GPS, GLONASS, Galileo, BeiDou, NavIC and QZSS              | 

## Roadmap

Until version 1.0 refactoring and renaming of crate's code elements is likely 
to happen. The following table outlines the high-level changes that are going 
to be inclided in the future version. Prioritisation is based on estimated 
significance and implementation effort of each item.

|Version |Category    |Content                                                |
|--------|------------|-------------------------------------------------------|
|0.5     |AIS         |VDM/VDO types 9-17                                     |
|0.6     |GNSS        |ALM, DTM, GBS, HDT, ROT, STN, TRF, VBW, ZDA, XTC, XTE  |
|0.7     |AIS         |VDM/VDO types 20, 22, 23, 25 and 26                    |
|0.8     |AIS         |VDM/VDO types 6-8                                      |
|1.0     |general     |Stable API, optimizations, enhanced documentation      |
|1.1     |GNSS        |AAM, BOD, BWC, R00, RMB, RTE, WPL, ZTG                 |
|1.2     |GNSS        |APB, RMA, GRS, GST, MSK, MSS, STN, VBW                 |

## Minimum Rust version

The crate's minimum supported Rust toolchain version is 1.44.

## License

This crate is licensed under [Apache 2.0 license] which also includes the 
liability and warranty statements.

[changelog]: CHANGELOG.md
[Apache 2.0 license]: LICENSE
[Rust]: https://en.wikipedia.org/wiki/Rust_(programming_language)
[AIS]: https://en.wikipedia.org/wiki/Automatic_identification_system
[GNSS]: https://en.wikipedia.org/wiki/Satellite_navigation
[NMEA 0183]: https://en.wikipedia.org/wiki/NMEA_0183
