use std::fmt::{Display, Formatter, Result};

use crate::error::{plist::PlistParseError, streamtyped::StreamTypedError};

#[derive(Debug)]
pub enum MessageError {
    MissingData,
    NoText,
    StreamTypedParseError(StreamTypedError),
    PlistParseError(PlistParseError),
    InvalidTimestamp(i64),
    TimestampDiffError(i64, i64),
}

impl Display for MessageError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            MessageError::MissingData => write!(fmt, "No attributedBody found!"),
            MessageError::NoText => write!(fmt, "Message has no text!"),
            MessageError::StreamTypedParseError(why) => {
                write!(fmt, "Failed to parse attributedBody: {why}")
            }
            MessageError::PlistParseError(why) => {
                write!(fmt, "Failed to parse plist data: {why}")
            }
            MessageError::InvalidTimestamp(when) => {
                write!(fmt, "Timestamp is invalid: {when}")
            }
            MessageError::TimestampDiffError(start, end) => {
                write!(fmt, "Unable to format diff: {start} - {end}")
            }
        }
    }
}
