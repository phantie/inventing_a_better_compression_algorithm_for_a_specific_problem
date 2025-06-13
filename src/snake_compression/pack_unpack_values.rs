use super::defs::direction::Direction;

type ValuesInPenultimateByte = u8;

// packs sequence of directions to sequence of bytes
//
// each Direction is encoded using 2 bits because there are 4 values
// 4 directions can be encoded using 1 byte
//
// since last partition of directions can contain 1 to 4 values
// serializer pads such byte with zeroes
// deserializer requires to know how many directions were encoded in the last byte
//
pub fn pack_values(values: &[Direction]) -> (Vec<u8>, ValuesInPenultimateByte) {
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

    let values_in_penultimate_byte = {
        let remainder = (values.len() % 4) as u8;
        if remainder == 0 && !values.is_empty() {
            4
        } else {
            remainder
        }
    };

    (result, values_in_penultimate_byte)
}

pub fn unpack_values(
    bytes: &[u8],
    values_in_penultimate_byte: ValuesInPenultimateByte,
) -> Vec<Direction> {
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
            values_in_penultimate_byte
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
    fn test_pack_unpack_values() {
        let (packed, values_in_penultimate_byte) =
            pack_values(&[Direction::Right, Direction::Right, Direction::Down]);
        assert_eq!(packed, vec![0b11_11_01_00]);

        assert_eq!(
            unpack_values(&vec![0b11_11_01_00], values_in_penultimate_byte),
            vec![Direction::Right, Direction::Right, Direction::Down]
        );
    }
}

#[cfg(test)]
mod refactor_tests {
    use super::*;

    struct TestCase {
        directions: Vec<Direction>,
        expected_packing: Vec<u8>,
        expected_values_in_penultimate_byte: u8,
    }

    #[test]
    fn test_pack_unpack_values() {
        let case1 = TestCase {
            directions: vec![Direction::Right, Direction::Right, Direction::Down],
            expected_packing: vec![0b11_11_01_00],
            expected_values_in_penultimate_byte: 3,
        };

        let case2 = TestCase {
            directions: vec![
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ],
            expected_packing: vec![0b11_11_01_10],
            expected_values_in_penultimate_byte: 4,
        };

        let case3 = TestCase {
            directions: vec![],
            expected_packing: vec![],
            expected_values_in_penultimate_byte: 0,
        };

        let case4 = TestCase {
            directions: vec![
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ],
            expected_packing: vec![0b11_01_10_10, 0b01_11_00_00],
            expected_values_in_penultimate_byte: 2,
        };

        for case in [case1, case2, case3, case4] {
            let (packed, values_in_penultimate_byte) = pack_values(&case.directions);
            assert_eq!(packed.as_slice(), case.expected_packing.as_slice());
            assert_eq!(
                values_in_penultimate_byte,
                case.expected_values_in_penultimate_byte
            );

            assert_eq!(
                unpack_values(packed.as_slice(), values_in_penultimate_byte).as_slice(),
                case.directions.as_slice()
            );
        }
    }
}
