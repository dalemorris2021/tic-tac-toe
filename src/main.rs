use std::io::Write;

use crate::game_board::*;

pub mod game_board;

fn main() {
    let mut game_board = GameBoard::new();
    let mut gaming = true;
    let mut turn = Symbol::X;
    while gaming {
        println!("{game_board}");
        let mut line = String::new();

        print!("Row: ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut line).unwrap();
        let row = line.trim().parse::<usize>().unwrap() - 1;
        line.clear();

        print!("Column: ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut line).unwrap();
        let column = line.trim().parse::<usize>().unwrap() - 1;
        line.clear();

        game_board.put_symbol(turn, (row, column));

        let winner = check_win(&game_board);
        match winner {
            Option::None => (),
            Option::Some(symbol) => {
                println!("{symbol} wins!");
                gaming = false;
            }
        };

        turn = match turn {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
        }
    }
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
