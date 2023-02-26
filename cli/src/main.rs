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

        match catch_input(16)? {
            KeyCode::Char('q') => {
                game.state = GameState::Lost;
            }
            KeyCode::Char('w') => {
                game.move_player(1, -1.0);
            }
            KeyCode::Char('s') => {
                game.move_player(1, 1.0);
            }
            /*
            KeyCode::Up => {
                game.move_player(2, -1.0);
            }
            KeyCode::Down => {
                game.move_player(2, 1.0);
            }
            */
            _ => {}
        }

        /*
        if game.player1.position < game.ball.1 {
            game.move_player(1, 1.0);
        } else if game.player1.position > game.ball.1 {
            game.move_player(1, -1.0);
        }
        */

        if game.player2.position < game.ball.1 {
            game.move_player(2, 0.15);
        } else if game.player2.position > game.ball.1 {
            game.move_player(2, -0.15);
        }
    }

    Ok(())
}
