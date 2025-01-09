use std::fmt::{Display, Formatter};
use std::str::FromStr;

/*
Chunk Type，4 个字节的块类型代码。
类型代码仅限于由大写和小写 ASCII 字母（A-Z 和 a-z，或十进制 65-90 和 97-122）组成

辅助位：第一个字节的第 5 位：0（大写）= critical(关键)，1（小写）= ancillary(辅助)
专用位：第二个字节的第 5 位：0（大写）= public，1（小写）= private
保留位：第三个字节的第 5 位：在符合此版本 PNG 的文件中必须为 0（大写）
安全复制位：第四字节第 5 位：0（大写）= 复制不安全，1（小写）= 复制安全
 */
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct ChunkType(u8, u8, u8, u8);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !value.iter().all(|&x| x.is_ascii_alphabetic()) {
            return Err("Byte must be A-Z or a-z");
        }
        Ok(Self(value[0], value[1], value[2], value[3]))
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("String length must be 4");
        }
        let value = s.as_bytes();
        if !value.iter().all(|&x| x.is_ascii_alphabetic()) {
            return Err("Byte must be A-Z or a-z");
        }
        Ok(Self(value[0], value[1], value[2], value[3]))
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.0 as char, self.1 as char, self.2 as char, self.3 as char
        )
    }
}

impl ChunkType {
    pub(crate) fn bytes(&self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }

    fn is_valid(&self) -> bool {
        self.2.is_ascii_uppercase()
    }

    fn is_critical(&self) -> bool {
        self.0.is_ascii_uppercase()
    }

    fn is_public(&self) -> bool {
        self.1.is_ascii_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.2.is_ascii_uppercase()
    }

    fn is_safe_to_copy(&self) -> bool {
        self.3.is_ascii_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
