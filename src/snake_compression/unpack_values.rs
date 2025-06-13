use super::defs::direction::Direction;

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
    fn test_unpack_values() {
        assert_eq!(
            unpack_values(&vec![0b00_11_01_00], 3),
            vec![Direction::Up, Direction::Right, Direction::Down]
        );
    }
}
