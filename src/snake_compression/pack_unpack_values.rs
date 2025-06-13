use super::defs::direction::Direction;

type DirectionsInLastByte = u8;

// packs a sequence of directions into a sequence of bytes
//
// each Direction is encoded using 2 bits because there are 4 values
// 4 directions can be encoded in 1 byte
//
// since the last partition of directions can contain 1 to 4 values,
// the serializer pads such byte with zeroes
//
// returns a pair of packed values and how many directions are in the last byte
pub fn pack_values(directions: &[Direction]) -> (Vec<u8>, DirectionsInLastByte) {
    let mut result = Vec::with_capacity((directions.len() + 3) / 4);

    for chunk in directions.chunks(4) {
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

    let directions_in_last_byte = {
        let remainder = (directions.len() % 4) as u8;
        if remainder == 0 && !directions.is_empty() {
            4
        } else {
            remainder
        }
    };

    (result, directions_in_last_byte)
}

pub fn unpack_values(
    packed: &[u8],
    directions_in_last_byte: DirectionsInLastByte,
) -> Vec<Direction> {
    let mut result = Vec::with_capacity(packed.len() * 4);

    fn decode_byte(byte: u8, contains: u8) -> Vec<Direction> {
        let mut result = vec![];

        assert!(contains >= 1);
        assert!(contains <= 4);

        for i in 0..contains {
            let mask_shift = 6 - (2 * i);

            let mask = 0b11 << mask_shift;

            // extract bits using:
            // shifted 0b11 with & (removing bits to the left and right of the mask),
            // then bit shift to the right by the mask shift size,
            // leaving you with a byte not exceeding decimal value 3 (2 bits)
            let dir_encoded = (byte & mask) >> mask_shift;
            result.push(Direction::decode(dir_encoded).expect("to be packed properly"));
        }

        result
    }

    for (i, byte) in packed.into_iter().enumerate() {
        let contains = if i == packed.len() - 1 {
            directions_in_last_byte
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
        let (packed, values_in_last_byte) =
            pack_values(&[Direction::Right, Direction::Right, Direction::Down]);
        assert_eq!(packed, vec![0b11_11_01_00]);

        assert_eq!(
            unpack_values(&vec![0b11_11_01_00], values_in_last_byte),
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
        expected_directions_encoded_in_last_byte: u8,
    }

    #[test]
    fn test_pack_unpack_values() {
        let case1 = TestCase {
            directions: vec![Direction::Right, Direction::Right, Direction::Down],
            expected_packing: vec![0b11_11_01_00],
            expected_directions_encoded_in_last_byte: 3,
        };

        let case2 = TestCase {
            directions: vec![
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ],
            expected_packing: vec![0b11_11_01_10],
            expected_directions_encoded_in_last_byte: 4,
        };

        let case3 = TestCase {
            directions: vec![],
            expected_packing: vec![],
            expected_directions_encoded_in_last_byte: 0,
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
            expected_directions_encoded_in_last_byte: 2,
        };

        for case in [case1, case2, case3, case4] {
            let (packed, values_in_last_byte) = pack_values(&case.directions);
            assert_eq!(packed.as_slice(), case.expected_packing.as_slice());
            assert_eq!(
                values_in_last_byte,
                case.expected_directions_encoded_in_last_byte
            );

            assert_eq!(
                unpack_values(packed.as_slice(), values_in_last_byte).as_slice(),
                case.directions.as_slice()
            );
        }
    }
}
