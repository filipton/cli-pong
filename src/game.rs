use std::time::Instant;

const BALL_SPEED: f32 = 5.0;
const MAP_LIMIT: (usize, usize) = (40, 20);

pub struct GameData {
    pub ball: (f32, f32),
    ball_direction: (f32, f32),

    timer: Instant,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            ball: (20.0, 10.0),
            ball_direction: (-1.0, -0.5),

            timer: Instant::now(),
        }
    }

    pub fn print(&self) {
        println!("{:?}\r", self.timer.elapsed());
        println!("+{}+\r", "-".repeat(MAP_LIMIT.0));
        for y in 0..=MAP_LIMIT.1 {
            print!("|");
            for x in 0..MAP_LIMIT.0 {
                print!(" ");
            }
            println!("|\r");
        }

        println!("+{}+\r", "-".repeat(MAP_LIMIT.0));
    }
}
