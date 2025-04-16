//
// author: xigang
//

use std::fmt;
use crate::chunk_type_error::ChunkTypeError;

#[derive(Debug)]
pub enum ChunkError {
    InvalidByte,
    InvalidLength,
    InvalidChunkType(ChunkTypeError),
    InvalidCRC,
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChunkError::InvalidByte       => write!(f, "Invalid byte in input"),
            ChunkError::InvalidLength  => write!(f, "Invalid length of input"),
            ChunkError::InvalidChunkType(cte) => write!(f, "Invalid chunk type: {}", cte),
            ChunkError::InvalidCRC => write!(f, "Invalid CRC in input"),
        }
    }
}

impl std::error::Error for ChunkError {}
