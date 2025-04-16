// 
// author: xigang zhao
//

use std::convert::TryFrom;
use std::fmt;
use crc32fast::hash;
use crc::{Crc, CRC_32_ISO_HDLC};
use crate::chunk_type::ChunkType;

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
        let size = value.len();
        if size >= Chunk::CHUNK_MINIMUM {
            let (blength, value) = value.split_at(Chunk::CHUNK_LENGTH_BYTES);
            let length = u32::from_be_bytes(blength.try_into().unwrap());
            let ulength: usize = length.try_into().unwrap();
            if ulength + Chunk::CHUNK_MINIMUM > size {
                return Err("Invalid input!");
            }
            let (bchunktype, value) = value.split_at(Chunk::CHUNK_TYPE_BYTES);
            let bchunktype: [u8; 4] = bchunktype.try_into().unwrap();
            let chunktype = match ChunkType::try_from(bchunktype) {
                Ok(ChunkType) => ChunkType,
                Err(err) => return Err("Invalid input!"),
            };
            let (chunkdata, value) = value.split_at(ulength);
            let (bcrc, _) = value.split_at(Chunk::CHUNK_CRC_BYTES);
            let bcrc: [u8; 4] = bcrc.try_into().unwrap();
            let bytes_verify: Vec<u8> = bchunktype.iter()
                                                       .chain(chunkdata.iter())
                                                       .copied()
                                                       .collect();
            let crc_read = u32::from_be_bytes(bcrc);
            let crc_obj = Crc::<u32>::new(&CRC_32_ISO_HDLC);

            let crc_verify = crc_obj.checksum(&bytes_verify);
            dbg!(crc_verify, crc_read);
            if crc_read != crc_verify {
                return Err("Invalid crc!");
            }
            return Ok(Chunk { length: length, chunktype: chunktype, chunkdata: chunkdata.to_vec(), crc: crc_read });
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
    pub const CHUNK_LENGTH_BYTES: usize = 4;
    pub const CHUNK_TYPE_BYTES:   usize = 4;
    pub const CHUNK_CRC_BYTES:    usize = 4;
    pub const CHUNK_MINIMUM: usize = Chunk::CHUNK_CRC_BYTES + Chunk::CHUNK_LENGTH_BYTES + Chunk::CHUNK_TYPE_BYTES;

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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = self.length.to_be_bytes().to_vec();
        res.extend_from_slice(&self.chunktype.bytes().to_vec());
        res.extend_from_slice(&self.chunkdata);
        res.extend_from_slice(&self.crc.to_be_bytes().to_vec());
        return res;
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
        dbg!(chunk.crc());
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