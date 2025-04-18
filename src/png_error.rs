//
// author: xigang
//

use std::fmt;
use crate::chunk_type_error::ChunkTypeError;
use crate::chunk_error::ChunkError;

#[derive(Debug)]
pub enum PngError {
    InvalidHeader,
    InvalidByte,
    InvalidLength,
    InvalidEncodeType,
    InvalidChunkType(ChunkTypeError),
    InvalidCRC,
    InvalidChunk(ChunkError),
    UnknownChunkType,
}

impl fmt::Display for PngError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PngError::InvalidHeader    => write!(f, "Invalid header in input"),
            PngError::InvalidByte      => write!(f, "Invalid byte in input"),
            PngError::InvalidLength    => write!(f, "Invalid length of input"),
            PngError::InvalidEncodeType => write!(f, "Invalid encoding type"),
            PngError::InvalidChunkType(cte) => write!(f, "Invalid chunk type: {}", cte),
            PngError::InvalidChunk(ce) => write!(f, "Invalid chunk: {}", ce),
            PngError::InvalidCRC       => write!(f, "Invalid CRC in input"),
            PngError::UnknownChunkType => write!(f, "Unknown chunk type"),
        }
    }
}

impl std::error::Error for PngError {}
