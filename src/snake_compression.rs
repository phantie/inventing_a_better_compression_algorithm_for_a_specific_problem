use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Bottom,
    Left,
    Right,
}

impl Direction {
    pub fn encode(&self) -> u8 {
        match self {
            Self::Up => 0b00,
            Self::Bottom => 0b01,
            Self::Left => 0b10,
            Self::Right => 0b11,
        }
    }

    pub fn decode(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::Up),
            0b01 => Some(Self::Bottom),
            0b10 => Some(Self::Left),
            0b11 => Some(Self::Right),
            _ => None,
        }
    }
}

// packs sequence of directions to sequence of bytes
//
// each Direction is encoded using 2 bits because there are 4 values
// 4 directions can be encoded using 1 byte
//
// since last partition of directions can contain 1 to 4 values
// serializer pads such byte with zeroes
// deserializer requires to know how many directions to decode the last byte
//
pub fn pack_values(values: &[Direction]) -> Vec<u8> {
    let mut result = Vec::with_capacity((values.len() + 3) / 4);

    for chunk in values.chunks(4) {
        // start with empty byte
        let mut byte = 0u8;

        for dir in chunk {
            // move to left, leaving 2 bits of space
            byte <<= 2;
            // use bit OR to append 2 bit value to the end
            byte |= dir.encode();
        }

        // pad zeroes when chunk length is less than 4
        byte <<= 2 * (4 - chunk.len());

        result.push(byte);
    }

    result
}

pub fn unpack_values(bytes: &[u8], values_in_last_byte: u8) -> Vec<Direction> {
    let mut result = Vec::with_capacity(bytes.len() * 4);

    fn decode_byte(byte: u8, contains: u8) -> Vec<Direction> {
        let mut result = vec![];

        assert!(contains >= 1);
        assert!(contains <= 4);

        for i in 0..contains {
            let mask_shift = 6 - (2 * i);

            let mask = 0b11 << mask_shift;

            // extract bits using:
            // shifted 0b11 with & (removing bits to the left and right of the mask)
            // and then bit shift to the right to mask shift size
            // leaving you with a byte not exceeding decimal value 4 (2 bits)
            let dir_encoded = (byte & mask) >> mask_shift;
            result.push(Direction::decode(dir_encoded).unwrap()); // TODO handle unwrap
        }

        result
    }

    for (i, byte) in bytes.into_iter().enumerate() {
        let contains = if i == bytes.len() - 1 {
            values_in_last_byte
        } else {
            4u8
        };

        result.extend(decode_byte(*byte, contains));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_values() {
        assert_eq!(
            pack_values(&[Direction::Up, Direction::Right, Direction::Bottom]),
            vec![0b00_11_01_00]
        );
    }

    #[test]
    fn test_unpack_values() {
        assert_eq!(
            unpack_values(&vec![0b00_11_01_00], 3),
            vec![Direction::Up, Direction::Right, Direction::Bottom]
        );
    }
}
