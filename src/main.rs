use crossterm::{terminal, execute};

pub struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen)
            .expect("Unable to leave alternate screen");
    }
}

struct Player {
    pub palette_pos: u32,
}

struct Board {
    pub player1: Player,
    pub player2: Player,

    pub ball_pos: (i32, i32),
    pub ball_dir: (i32, i32),

    pub limit: (i32, i32),
}

fn main() {
    let mut board: Board = Board {
        player1: Player { palette_pos: 0 },
        player2: Player { palette_pos: 0 },
        ball_pos: (0, 5),
        ball_dir: (1, 1),
        limit: (20, 20),
    };

    print_board(&board);
    loop {
        update_board(&mut board);
        print_board(&board);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn print_board(board: &Board) {
    for y in 0..board.limit.1 {
        for x in 0..board.limit.0 {
            if x == board.ball_pos.0 && y == board.ball_pos.1 {
                print!("o");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn update_board(board: &mut Board) {
    board.ball_pos.0 += board.ball_dir.0;
    board.ball_pos.1 += board.ball_dir.1;

    let (x, y) = board.ball_pos;

    if x >= board.limit.0 || x <= 0 {
        board.ball_dir.0 = -board.ball_dir.0;
    }
    if y >= board.limit.1 || y <= 0 {
        board.ball_dir.1 = -board.ball_dir.1;
    }
}
