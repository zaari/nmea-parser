# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Implementation for AIS VDM/VDO sentence type 11 parsing
- Implementation for AIS VDM/VDO sentence type 10 parsing
- Implementation for AIS VDM/VDO sentence type 9 parsing
### Changed
- Sentence checksum length limited to two characters (as some messages may have non-standard extensions)
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
- Renamed `NmeaStore` to `NmeaParser` and made `parse_sentence` its member function
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

