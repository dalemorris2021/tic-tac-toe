use std::fmt;

pub enum Option<T> {
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

pub type Tile = Option<Symbol>;

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

pub struct Row {
    tiles: [Tile; 3],
}

impl Row {
    fn new() -> Self {
        let tiles = [Option::None, Option::None, Option::None];
        Self { tiles }
    }
}

pub struct GameBoard {
    rows: [Row; 3],
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = &self.rows;
        let repr = format!(
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            rows[0].tiles[0],
            rows[0].tiles[1],
            rows[0].tiles[2],
            rows[1].tiles[0],
            rows[1].tiles[1],
            rows[1].tiles[2],
            rows[2].tiles[0],
            rows[2].tiles[1],
            rows[2].tiles[2],
        );
        write!(f, "{repr}",)
    }
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let rows = [Row::new(), Row::new(), Row::new()];
        GameBoard { rows }
    }

    pub fn put_symbol(&mut self, symbol: Symbol, point: Point) {
        self.rows[point.0].tiles[point.1] = Option::Some(symbol);
    }
}
