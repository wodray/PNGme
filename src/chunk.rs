use crate::chunk_type::ChunkType;
use anyhow::Result;
use std::fmt::{Display, Formatter};

const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

/*
CRC：对数据块前面的字节计算的4字节CRC（循环冗余校验），包括数据块类型代码和数据块数据字段，但不包括长度字段。
 */
struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    // 字节表示：4 + 4 + len + 4
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (len_bytes, rest) = value
            .split_at_checked(4)
            .ok_or("Length bytes length must be 4")?;
        let length = u32::from_be_bytes(
            len_bytes
                .try_into()
                .map_err(|_| "Invalid length bytes into u32")?,
        );

        let (chunk_type_bytes, rest) = rest
            .split_at_checked(4)
            .ok_or("Chunk type bytes length must be 4")?;
        let chunk_type_arr: [u8; 4] = chunk_type_bytes
            .try_into()
            .map_err(|_| "Invalid chunk type bytes into [u8; 4]")?;
        let chunk_type = ChunkType::try_from(chunk_type_arr)?;

        let (data_bytes, rest) = rest
            .split_at_checked(length as usize)
            .ok_or("Data bytes must match given length")?;

        let (crc_bytes, rest) = rest
            .split_at_checked(4)
            .ok_or("CRC bytes length must be 4")?;
        if rest.len() != 0 {
            return Err("Invalid bytes length");
        }
        let input_crc = u32::from_be_bytes(
            crc_bytes
                .try_into()
                .map_err(|_| "Invalid CRC bytes into u32")?,
        );
        let preceding_bytes = [chunk_type_bytes, data_bytes].concat();
        let calculated_crc = CRC.checksum(&preceding_bytes);
        if input_crc != calculated_crc {
            return Err("Invalid given CRC");
        }

        Ok(Self {
            length,
            chunk_type,
            data: data_bytes.to_vec(),
            crc: input_crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ length: {}, chunk_type: \"{}\", data: {}, crc: {} }}",
            self.length,
            self.chunk_type,
            self.data_as_string().map_err(|_| std::fmt::Error)?,
            self.crc
        )
    }
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let preceding_bytes = [chunk_type.bytes().as_ref(), &data].concat();
        let crc = CRC.checksum(&preceding_bytes);
        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.data.to_vec())?)
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
