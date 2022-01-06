# NMEA Parser for Rust

[![NMEA Parser on crates.io][cratesio-image]][cratesio]
[![NMEA Parser on docs.rs][docsrs-image]][docsrs]
[![GitHub last commit][ghcommit-image]][ghcommit]

[cratesio-image]: https://img.shields.io/crates/v/nmea-parser.svg
[cratesio]: https://crates.io/crates/nmea-parser
[docsrs-image]: https://docs.rs/nmea-parser/badge.svg
[docsrs]: https://docs.rs/nmea-parser
[ghcommit-image]: https://img.shields.io/github/last-commit/zaari/nmea-parser
[ghcommit]: https://github.com/zaari/nmea-parser/

This [Rust] crate aims to cover all [AIS] sentences and the most important [GNSS] sentences used 
with [NMEA 0183] standard. It supports both AIS class A and class B.

## Usage

Include the following fragment in your `Cargo.toml` file:

```toml
[dependencies]
nmea-parser = "0.9.0"
```

The following example code fragment uses the crate to parse the given NMEA sentences and to print 
some parsed fields. It relies on `unwrap()` function to simplify the example. In real-life 
applications proper handling of `None` cases is needed.

```rust
use nmea_parser::*;

// Create parser and define sample sentences
let mut parser = NmeaParser::new();
let sentences = vec![
  "!AIVDM,1,1,,A,H42O55i18tMET00000000000000,2*6D",
  "!AIVDM,1,1,,A,H42O55lti4hhhilD3nink000?050,0*40",
  "$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*56",
];

// Parse the sentences and print some fields of the messages
for sentence in sentences {    
    match parser.parse_sentence(sentence)? {
        ParsedMessage::VesselDynamicData(vdd) => {
            println!("MMSI:    {}",        vdd.mmsi);
            println!("Speed:   {:.1} kts", vdd.sog_knots.unwrap());
            println!("Heading: {}°",       vdd.heading_true.unwrap());
            println!("");
        },
        ParsedMessage::VesselStaticData(vsd) => {
            println!("MMSI:  {}", vsd.mmsi);
            println!("Flag:  {}", vsd.country().unwrap());
            println!("Name:  {}", vsd.name.unwrap());
            println!("Type:  {}", vsd.ship_type);
            println!("");
        },
        ParsedMessage::Gga(gga) => {
            println!("Source:    {}",     gga.source);
            println!("Latitude:  {:.3}°", gga.latitude.unwrap());
            println!("Longitude: {:.3}°", gga.longitude.unwrap());
            println!("");
        },
        ParsedMessage::Rmc(rmc) => {
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

The example outputs the following lines:

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

The following features are included in the published version of the crate. Details about version 
history can be found from the [changelog].

|Feature          |Description                                                     |
|-----------------|----------------------------------------------------------------|
|AIS sentences    |VDM/VDO types 1-5, 9-27                                         |
|GNSS sentences   |ALM, DBS, DPT, DTM, GGA, GLL, GNS, GSA, GSV, HDT, MTW, MWV, RMC, VTG, MSS, STN, VBW, VHW, ZDA |
|Satellite systems|GPS, GLONASS, Galileo, BeiDou, NavIC and QZSS                   | 

## Roadmap

The following table outlines the high-level changes that are going to be included in the future 
versions. Prioritization is based on estimated significance and implementation effort of each item. 
Until version 1.0 refactoring and renaming of code elements is likely to happen. 

|Version |Category    |Content                                                   |
|--------|------------|----------------------------------------------------------|
|0.10    |AIS         |VDM/VDO types 6-8                                         |
|1.0     |general     |Stable API, optimizations, documentation enhancements, even more unit tests, examples|
|1.1     |GNSS        |AAM, BOD, BWC, HDT, R00, RMB, ROT, RTE, WPL, ZTG, APB, GBS, RMA, GRS, GST, MSK, STN, VBW, XTE, XTR|

## Minimum Rust version

The crate's minimum supported Rust toolchain version is 1.43.

## License

This crate is licensed under [Apache 2.0 license] which also includes the liability and warranty 
statements.

[changelog]: CHANGELOG.md
[Apache 2.0 license]: LICENSE
[Rust]: https://en.wikipedia.org/wiki/Rust_(programming_language)
[AIS]: https://en.wikipedia.org/wiki/Automatic_identification_system
[GNSS]: https://en.wikipedia.org/wiki/Satellite_navigation
[NMEA 0183]: https://en.wikipedia.org/wiki/NMEA_0183
