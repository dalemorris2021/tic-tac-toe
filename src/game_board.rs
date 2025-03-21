use std::fmt;

enum Option<T> {
    None,
    Some(T),
}

pub enum Symbol {
    X,
    O,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Symbol::X => "X",
            Symbol::O => "O",
        };
        write!(f, "{repr}")
    }
}

type Tile = Option<Symbol>;

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = match self {
            Option::None => " ".to_owned(),
            Option::Some(symbol) => format!("{symbol}"),
        };
        write!(f, "{repr}")
    }
}

type Point = (usize, usize);

pub struct GameBoard {
    rows: [[Tile; 3]; 3],
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = &self.rows;
        let repr = format!(
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            rows[0][0],
            rows[0][1],
            rows[0][2],
            rows[1][0],
            rows[1][1],
            rows[1][2],
            rows[2][0],
            rows[2][1],
            rows[2][2],
        );
        write!(f, "{repr}",)
    }
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let rows = [
            [Option::None, Option::None, Option::None],
            [Option::None, Option::None, Option::None],
            [Option::None, Option::None, Option::None],
        ];
        GameBoard { rows }
    }

    pub fn put_symbol(&mut self, symbol: Symbol, point: Point) {
        self.rows[point.0][point.1] = Option::Some(symbol);
    }
}
