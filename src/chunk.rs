// 
// author: xigang zhao
//

use std::convert::TryFrom;
use std::fmt;
use crc32fast::hash;
use crate::chunk_type;
use super::chunk_type::ChunkType;


#[derive(Debug, PartialEq, Eq)]
pub struct Chunk {
    length    : u32,
    chunktype : ChunkType,
    chunkdata : Vec<u8>,
    crc       : u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let size: u32 = value.len().try_into().unwrap();
        if size >= 12 {
            let length = u32::from(value[0]) << 24 | u32::from(value[1]) << 16 |
                              u32::from(value[2]) << 8  | u32::from(value[3]);
            if length + 12 > size {
                return Err("Invalid input!");
            }
            let chunktype = ChunkType::try_from([82, 117, 83, 116]).unwrap();
            let length_usize: usize = length.try_into().unwrap();
            let chunkdata = Vec::from(&value[8..(8+length_usize)]);
            let crc = u32::from(value[8+length_usize]) << 24  | u32::from(value[9+length_usize]) << 16 |
                           u32::from(value[10+length_usize]) << 8  | u32::from(value[11+length_usize]);
            return Ok(Chunk { length: length, chunktype: chunktype, chunkdata: chunkdata, crc: crc });
        }
        return Err("Invalid input!");
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::from("Unimplemented");
        write!(f, "{}", s)
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let size: u32 = data.len().try_into().unwrap();
        let mut raw_bytes = chunk_type.bytes().to_vec().clone();
        raw_bytes.extend_from_slice(&data);
        let crc: u32 = hash(&raw_bytes);
        return Chunk{length: size, chunktype: chunk_type, chunkdata: data, crc: crc};
    }

    pub fn length(&self) -> u32 {
        return self.length;
    }

    pub fn chunk_type(&self) -> &ChunkType {
        return &self.chunktype;
    }

    pub fn data(&self) -> &[u8] {
        return &self.chunkdata.as_slice();
    }

    pub fn crc(&self) -> u32 {
        return self.crc;
    }

    pub fn data_as_string(&self) -> Result<String, std::str::Utf8Error> {
        match std::str::from_utf8(&self.chunkdata) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}