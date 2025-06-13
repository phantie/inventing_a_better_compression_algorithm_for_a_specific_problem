use super::defs::direction::Direction;

// packs sequence of directions to sequence of bytes
//
// each Direction is encoded using 2 bits because there are 4 values
// 4 directions can be encoded using 1 byte
//
// since last partition of directions can contain 1 to 4 values
// serializer pads such byte with zeroes
// deserializer requires to know how many directions were encoded in the last byte
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_values() {
        assert_eq!(
            pack_values(&[Direction::Right, Direction::Right, Direction::Down]),
            vec![0b11_11_01_00]
        );
    }
}
