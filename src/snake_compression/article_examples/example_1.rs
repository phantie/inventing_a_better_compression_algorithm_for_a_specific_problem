use crate::snake_compression::defs::direction::Direction;
use crate::snake_compression::defs::pos::Pos;

fn foo() {
    {
        let positions: &[Pos] = [
            Pos { x: 0, y: 0 },
            Pos { x: 1, y: 1 },
            Pos { x: 2, y: 2 },
            Pos { x: 3, y: 3 },
        ]
        .as_slice();
    }
    {
        let as_tuple: (i32, i32) = (0, 0);
    }
    {
        let starting_position: Pos = Pos { x: 2, y: 1 };
        let consequent_directions =
            [Direction::Right, Direction::Right, Direction::Down].as_slice();
    }
}
