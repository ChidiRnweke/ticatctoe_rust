use super::squares::{Board, ColumnTarget, TileFill, ValidMove};
use crate::logic::squares::RowTarget;
use ansi_term::Colour::Green;

use std::{fmt, io};

pub struct Game {
    pub board: Board,
    current_player: TileFill,
}

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_) => {
            println!("Invalid input. Try again.");
            read_input()
        }
        _ => input,
    }
}

impl Game {
    pub fn start_game() {
        let game = Self {
            board: Board::default(),
            current_player: TileFill::O,
        };
        println!("Player {}'s turn.", TileFill::O);
        println!("{game}");
        Self::game_loop(game.next_turn());
    }

    fn game_loop(maybe_next: Option<Self>) {
        if let Some(not_finished) = maybe_next {
            println!("Player {}'s turn.", &not_finished.current_player);
            println!("{not_finished}");
            let next = not_finished.next_turn();
            Self::game_loop(next);
        }
    }

    fn next_turn(&self) -> Option<Self> {
        let next_move = self.make_valid_move();
        let new_board = self.board.make_move(self.current_player, &next_move);
        if new_board.is_complete() {
            println!(
                "{} {:?}, {}",
                Green.paint("Congratulations player"),
                self.current_player,
                Green.paint("you win!")
            );
            println!("{new_board}");
            None
        } else if new_board.is_draw() {
            println!("The game ended in a draw.");
            None
        } else {
            Some(Self {
                board: new_board,
                current_player: self.current_player.get_next_player(),
            })
        }
    }

    fn make_move(&self) -> Option<ValidMove> {
        let column: ColumnTarget = Self::read_column();
        let row = Self::read_row();
        ValidMove::new(&self.board, row, column)
    }

    fn make_valid_move(&self) -> ValidMove {
        let potential_move = self.make_move();
        potential_move.map_or_else(
            || {
                println!("You must select a tile that is empty. Try again.");
                self.make_valid_move()
            },
            |valid_move| valid_move,
        )
    }

    fn read_column() -> ColumnTarget {
        println!("What column do you want to place the tile in?");
        let col = read_input();
        col.trim()
            .parse::<usize>()
            .map_err(|_| "The number you gave was invalid. Try again.")
            .and_then(ColumnTarget::try_from)
            .unwrap_or_else(|err| {
                println!("{err}");
                Self::read_column()
            })
    }

    fn read_row() -> RowTarget {
        println!("What row do you want to place the tile in?");
        let col = read_input();
        col.trim()
            .parse::<usize>()
            .map_err(|_| "The number you gave was invalid. Try again.")
            .and_then(RowTarget::try_from)
            .unwrap_or_else(|err| {
                println!("{err}");
                Self::read_row()
            })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.board)
    }
}
