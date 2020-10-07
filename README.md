# NMEA Parser for Rust

[![NMEA Parser on crates.io][cratesio-image]][cratesio]
[![NMEA Parser on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/nmea-parser.svg
[cratesio]: https://crates.io/crates/nmea-parser
[docsrs-image]: https://docs.rs/nmea-parser/badge.svg
[docsrs]: https://docs.rs/nmea-parser

This Rust crate aims to cover the most important AIS and GNSS sentences. Supports AIS class A and B types. Identifies GPS, GLONASS, Galileo, BeiDou, Navic and QZSS satellite systems. 

## Usage

Include the following fragment in your `Cargo.toml` file:

```toml
[dependencies]
nmea-parser = "0.1.0"
```

Sample example program:

```rust
use nmea_parser::*;

pub fn main() -> Result<(), String> {
    let mut store = NmeaStore::new();
    let sentence = "$GAGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*56";
    
    match nmea_parser::decode_sentence(sentence, &mut store)? {
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
    Ok(())
}
```

## Feature Roadmap

|Version |Category    |Goal                                                                 |
|--------|------------|---------------------------------------------------------------------|
|**0.1** |AIS, GNSS   |AIS VDM/VDO t1-3, t5, t18-19, t24, GGA, RMC, GSA, GSV, VTG, GLL      |
|0.2     |AIS         |AIS VDM/VDO t4, t6-t17, t20-t23, t25-27                              |
|1.0     |meta        |API freeze, enhanced documentation                                   |
|1.1     |GNSS        |ALM, TRF, STN, VBW, XTC, XTE, ZDA                                    |
|1.2     |GNSS, route |AAM, BOD, BWC, R00, RMB, RTE, WPL, ZTG                               |
|1.3     |GNSS, misc  |APB, DTM, RMA, GRS, GST, MSK, MSS, STN, VBW                          |

