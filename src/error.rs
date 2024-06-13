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

use core::fmt;
use core::num::{ParseIntError, ParseFloatError};
use alloc::string::String;

/// Parse error returned by `NmeaParser::parse_sentence()`. `String` data type is used instead of
/// `static &str` because the error messages are expected to contain context-specific details.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    /// Unsupported (or unimplemented) sentence type
    UnsupportedSentenceType(String),

    /// NMEA checksum doesn't match
    CorruptedSentence(String),

    /// The sentence format isn't what expected
    InvalidSentence(String),
}

impl From<String> for ParseError {
    fn from(s: String) -> Self {
        ParseError::InvalidSentence(s)
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidSentence(format!("{}", e))
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(e: ParseFloatError) -> Self {
        ParseError::InvalidSentence(format!("{}", e))
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnsupportedSentenceType(s) => {
                write!(f, "Unsupported NMEA sentence type: {}", s)
            }
            ParseError::CorruptedSentence(s) => write!(f, "Corrupted NMEA sentence: {}", s),
            ParseError::InvalidSentence(s) => write!(f, "Invalid NMEA sentence: {}", s),
        }
    }
}
