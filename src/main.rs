use crate::game_board::*;

pub mod game_board;

fn main() {
    let mut game_board = GameBoard::new();
    game_board.put_symbol(Symbol::X, (0, 0));
    game_board.put_symbol(Symbol::O, (0, 1));
    game_board.put_symbol(Symbol::X, (0, 2));
    game_board.put_symbol(Symbol::O, (1, 0));
    game_board.put_symbol(Symbol::X, (1, 1));
    game_board.put_symbol(Symbol::O, (1, 2));
    game_board.put_symbol(Symbol::X, (2, 0));
    game_board.put_symbol(Symbol::O, (2, 1));
    game_board.put_symbol(Symbol::X, (2, 2));

    println!("{game_board}");
}
