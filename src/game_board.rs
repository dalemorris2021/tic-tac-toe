use std::fmt;

#[derive(Clone, Copy)]
pub enum Option<T> {
    None,
    Some(T),
}

#[derive(Clone, Copy, PartialEq)]
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

pub type Point = (usize, usize);

pub struct GameBoard {
    tiles: [[Tile; 3]; 3],
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tiles = &self.tiles;
        let repr = format!(
            "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}",
            tiles[0][0],
            tiles[0][1],
            tiles[0][2],
            tiles[1][0],
            tiles[1][1],
            tiles[1][2],
            tiles[2][0],
            tiles[2][1],
            tiles[2][2],
        );
        write!(f, "{repr}",)
    }
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let tiles = [
            [Option::None, Option::None, Option::None],
            [Option::None, Option::None, Option::None],
            [Option::None, Option::None, Option::None],
        ];
        GameBoard { tiles }
    }

    pub fn get_tiles(&self) -> &[[Tile; 3]; 3] {
        &self.tiles
    }

    pub fn put_symbol(&mut self, symbol: Symbol, point: Point) {
        match self.tiles[point.0][point.1] {
            Option::None => self.tiles[point.0][point.1] = Option::Some(symbol),
            Option::Some(_) => (),
        };
    }
}
