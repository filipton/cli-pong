use std::time::Duration;

use anyhow::Result;
use crossterm::event::KeyCode;
use display::{catch_input, clear_console, move_zero, CleanUp};
use game::{GameData, GameState};

mod display;
mod game;

fn main() -> Result<()> {
    let _clean_up = CleanUp::new()?;
    let mut game: GameData = GameData::new();

    while game.state == GameState::Playing {
        move_zero();
        game.update();
        game.print();

        std::thread::sleep(Duration::from_millis(16));
    }

    /*
    while board.game_running {
        update_board(&mut board);

        clear_console();
        print_board(&board);

        match catch_input(100)? {
            KeyCode::Char('q') => {
                board.game_running = false;
            }
            KeyCode::Char('w') => {
                if board.player1.palette_pos > 0 {
                    board.player1.palette_pos -= 1;
                }
            }
            KeyCode::Char('s') => {
                if board.player1.palette_pos < board.limit.1 - 1 {
                    board.player1.palette_pos += 1;
                }
            }
            KeyCode::Up => {
                if board.player2.palette_pos > 0 {
                    board.player2.palette_pos -= 1;
                }
            }
            KeyCode::Down => {
                if board.player2.palette_pos < board.limit.1 - 1 {
                    board.player2.palette_pos += 1;
                }
            }
            _ => {}
        }
    }
    */

    Ok(())
}
