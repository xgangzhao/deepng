//
// author: xigang
//
use std::fmt;

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidByte,
    InvalidLength,
}

impl fmt::Display for ChunkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkTypeError::InvalidByte => write!(f, "Invalid byte found in input"),
            ChunkTypeError::InvalidLength     => write!(f, "Invalid length of input"),
        }
    }
}

impl std::error::Error for ChunkTypeError {}
