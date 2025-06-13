use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn encode(&self) -> u8 {
        match self {
            Self::Up => 0b00,
            Self::Down => 0b01,
            Self::Left => 0b10,
            Self::Right => 0b11,
        }
    }

    pub fn decode(value: u8) -> Option<Self> {
        match value {
            0b00 => Some(Self::Up),
            0b01 => Some(Self::Down),
            0b10 => Some(Self::Left),
            0b11 => Some(Self::Right),
            _ => None,
        }
    }
}
