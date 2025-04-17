//
// author: xigang
//

use std::fmt;
use std::string::FromUtf8Error;
use crate::chunk_type_error::ChunkTypeError;

#[derive(Debug)]
pub enum ChunkError {
    Utf8Error(FromUtf8Error),
    InvalidLength,
    InvalidChunkType(ChunkTypeError),
    InvalidCRC,
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkError::Utf8Error(_)  => write!(f, "Invalid UTF-8 data"),
            ChunkError::InvalidLength => write!(f, "Invalid length of input"),
            ChunkError::InvalidChunkType(cte) => write!(f, "Invalid chunk type: {}", cte),
            ChunkError::InvalidCRC    => write!(f, "Invalid CRC in input"),
        }
    }
}

impl std::error::Error for ChunkError {}
