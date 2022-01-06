# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
### Changed

## [0.9.0] - 2022-01-06
### Added
- Implementation for GNSS VHW parsing
- Implementation for GNSS MWV parsing
- Implementation for GNSS HDT parsing
- Test for bad talker ids
- Support for no_std use-cases, although an allocator is still required
### Changed
- Fixed a payload boundary issue for AIS message types 26 and 27"
- Fixed panic on propriatary messages
- For messages without a date, use 2000-01-01 instead of the current date
  (applies to GGA, GLL, GNS and AIVDM type 5)

## [0.8.0] - 2021-05-26
### Added
- Implementation for GNSS DBS parsing
- Implementation for GNSS DPT parsing
- Implementation for GNSS MTW parsing

## [0.7.2] - 2021-04-19
### Changed
- Documentation fix

## [0.7.1] - 2021-04-19
### Added
- Implementation for GNSS GNS parsing

## [0.7.0] - 2020-12-23
### Added
- Implementation for AIS VDM/VDO sentence type 26 parsing
- Implementation for AIS VDM/VDO sentence type 25 parsing
- Implementation for AIS VDM/VDO sentence type 23 parsing
- Implementation for AIS VDM/VDO sentence type 22 parsing
- Implementation for AIS VDM/VDO sentence type 20 parsing

## [0.6.0] - 2020-11-18
### Added
- Implementation for GNSS ZDA parsing
- Implementation for GNSS VBW parsing
- Implementation for GNSS STN parsing
- Implementation for GNSS MSS parsing
- Implementation for GNSS DTM parsing
- Implementation for GNSS ALM parsing
### Changed
- Forbade use of `unsafe` code

## [0.5.0] - 2020-10-31
### Added
- Implementation for AIS VDM/VDO sentence type 17 parsing
- Implementation for AIS VDM/VDO sentence type 16 parsing
- Implementation for AIS VDM/VDO sentence type 15 parsing
- Implementation for AIS VDM/VDO sentence type 14 parsing
- Implementation for AIS VDM/VDO sentence type 13 parsing
- Implementation for AIS VDM/VDO sentence type 12 parsing
- Implementation for AIS VDM/VDO sentence type 11 parsing
- Implementation for AIS VDM/VDO sentence type 10 parsing
- Implementation for AIS VDM/VDO sentence type 9 parsing
### Changed
- Sentence checksum length limited to two characters (as some messages may have extensions)
- Renamed `ParsedSentence` to `ParsedMessage`
- Changed type of `VesselDynamicData::rot_direction` from `i8` to `RotDirection`
- Proper datetime validation
- Re-export of `chrono` crate
- Turned crate `assert` into a dev-dependency
### Removed
- Dependency to `env_logger` removed

## [0.4.1] - 2020-10-15
### Changed
- Documentation corrections and changes

## [0.4.0] - 2020-10-15
### Added
- Implementation for AIS VDM/VDO sentence type 21 parsing
- Implementation for AIS VDM/VDO sentence type 27 parsing
- Implementation for AIS VDM/VDO sentence type 4 parsing
- Partial implementation for AIS VDM/VDO sentence type 6 parsing
- New field `current_gnss_position` added to `VesselDynamicData` struct
### Changed
- Renamed `NmeaStore` to `NmeaParser` and made `parse_sentence` as its member function
- Submodule documentation visibility fixes
- Type of `VesselDynamicData::radio_status` changed from `u32` to `Option<u32>` because type 27 
  sentences don't have the field
- Dependency `regex` upgraded to version 1.4

## [0.3.1] - 2020-10-09
### Changed
- Fixed the example program in README.md

## [0.3.0] - 2020-10-09
### Added
- `ParseError` type added

### Changed
- Renamed `gnss::*::system` field to `gnss::*::source`
- Changed `parse_sentence` to return `ParseError` instead of plain `String` in case of errors
- Renamed `RmcData::speed_knots` to `RmcData::sog_knots`
- Renamed `gnss` module's structs and enums to make them more consistent
- Refactored the whole module hierarchy to improve modularity and clarity
- Improved the example program

## [0.2.0] - 2020-10-08
### Changed
- Renamed `decode_sentence` to `parse_sentence` 

## [0.1.1] - 2020-10-07
### Added
- Missing code generation script `mid-to-iso3166.py` added
### Changed
- Minor refactoring and documentation corrections

## [0.1.0] - 2020-10-07
### Added
- Initial release

