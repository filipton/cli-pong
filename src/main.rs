use anyhow::Result;
use crossterm::{event, execute, terminal};

pub struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen)
            .expect("Unable to leave alternate screen");
    }
}

struct Player {
    pub palette_pos: i32,
    pub palette_size: i32,
}

struct Board {
    pub game_running: bool,

    pub player1: Player,
    pub player2: Player,

    pub ball_pos: (f32, f32),
    pub ball_dir: (f32, f32),

    pub limit: (i32, i32),
}

fn main() -> Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;
    execute!(std::io::stdout(), terminal::EnterAlternateScreen)?;

    let mut board: Board = Board {
        game_running: true,
        player1: Player {
            palette_pos: 0,
            palette_size: 6,
        },
        player2: Player {
            palette_pos: 0,
            palette_size: 6,
        },
        ball_pos: (0.0, 5.0),
        ball_dir: (1.0, 1.0),
        limit: (40, 20),
    };

    while board.game_running {
        update_board(&mut board);

        clear_console();
        print_board(&board);

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => {
                        board.game_running = false;
                    }
                    event::KeyCode::Char('w') => {
                        if board.player1.palette_pos > 0 {
                            board.player1.palette_pos -= 1;
                        }
                    }
                    event::KeyCode::Char('s') => {
                        if board.player1.palette_pos < board.limit.1 - 1 {
                            board.player1.palette_pos += 1;
                        }
                    }
                    event::KeyCode::Up => {
                        if board.player2.palette_pos > 0 {
                            board.player2.palette_pos -= 1;
                        }
                    }
                    event::KeyCode::Down => {
                        if board.player2.palette_pos < board.limit.1 - 1 {
                            board.player2.palette_pos += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn print_board(board: &Board) {
    for y in 0..=board.limit.1 {
        for x in 0..=board.limit.0 {
            print!("{}", get_board_pixel(board, x, y));
        }
        println!("\r");
    }
}

fn update_board(board: &mut Board) {
    board.ball_pos.0 += board.ball_dir.0;
    board.ball_pos.1 += board.ball_dir.1;

    let x = board.ball_pos.0 as i32;
    let y = board.ball_pos.1 as i32;

    if x >= board.limit.0 || x <= 0 {
        board.ball_dir.0 = -board.ball_dir.0;
    }
    if y >= board.limit.1 || y <= 0 {
        board.ball_dir.1 = -board.ball_dir.1;
    }
}

fn get_board_pixel(board: &Board, x: i32, y: i32) -> char {
    let pallet1_size = board.player1.palette_size / 2;
    let pallet1_range =
        (board.player1.palette_pos - pallet1_size)..(board.player1.palette_pos + pallet1_size);

    let pallet2_size = board.player2.palette_size / 2;
    let pallet2_range =
        (board.player2.palette_pos - pallet2_size)..(board.player2.palette_pos + pallet2_size);

    if x == 0 && pallet1_range.contains(&y) {
        return '1';
    }

    if x == board.limit.0 && pallet2_range.contains(&y) {
        return '2';
    }

    if x == board.ball_pos.0 as i32 && y == board.ball_pos.1 as i32 {
        return 'o';
    }

    return ' ';
}

pub fn clear_console() {
    execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )
    .expect("Error");
}
