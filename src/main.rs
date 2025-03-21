use crate::game_board::*;

pub mod game_board;

fn main() {
    let mut game_board = GameBoard::new();
    game_board.put_symbol(Symbol::X, (0, 0));
    game_board.put_symbol(Symbol::X, (0, 1));
    game_board.put_symbol(Symbol::O, (0, 2));
    game_board.put_symbol(Symbol::O, (1, 0));
    game_board.put_symbol(Symbol::X, (1, 1));
    game_board.put_symbol(Symbol::X, (1, 2));
    game_board.put_symbol(Symbol::X, (2, 0));
    game_board.put_symbol(Symbol::O, (2, 1));
    game_board.put_symbol(Symbol::O, (2, 2));
    println!("{game_board}");

    let winner = check_win(&game_board);
    match winner {
        Option::None => println!("No winner!"),
        Option::Some(symbol) => println!("{symbol} wins!"),
    };
}

fn check_win(game_board: &GameBoard) -> Option<Symbol> {
    let symbols = [Symbol::X, Symbol::O];

    for symbol in symbols {
        if check_win_symbol(game_board, &symbol) {
            return Option::Some(symbol);
        }
    }

    Option::None
}

fn check_win_symbol(game_board: &GameBoard, symbol: &Symbol) -> bool {
    let tiles = game_board.get_tiles();
    check_run(&[tiles[0][0], tiles[0][1], tiles[0][2]], symbol)
        || check_run(&[tiles[1][0], tiles[1][1], tiles[1][2]], symbol)
        || check_run(&[tiles[2][0], tiles[2][1], tiles[2][2]], symbol)
        || check_run(&[tiles[0][0], tiles[1][0], tiles[2][0]], symbol)
        || check_run(&[tiles[0][1], tiles[1][1], tiles[2][1]], symbol)
        || check_run(&[tiles[0][2], tiles[1][2], tiles[2][2]], symbol)
        || check_run(&[tiles[0][0], tiles[1][1], tiles[2][2]], symbol)
        || check_run(&[tiles[0][2], tiles[1][1], tiles[2][0]], symbol)
}

fn check_run(run: &[Tile; 3], symbol: &Symbol) -> bool {
    for tile in run {
        match tile {
            Option::None => return false,
            Option::Some(sym) => {
                if sym != symbol {
                    return false;
                }
            }
        }
    }

    true
}
