use std::time::Instant;

const MAP_LIMIT: (usize, usize) = (40, 20);

pub struct GameData {
    pub ball: (f32, f32),
    ball_direction: (f32, f32),

    last_update: u128,
    timer: Instant,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            ball: (20.0, 10.0),
            ball_direction: (-1.0, -0.5),

            last_update: 0,
            timer: Instant::now(),
        }
    }

    pub fn print(&self) {
        println!("{:?} {:?}\r", self.ball.0, self.ball.1);
        println!("+{}+\r", "-".repeat(MAP_LIMIT.0));
        for y in 0..=MAP_LIMIT.1 {
            print!("|");
            for x in 0..MAP_LIMIT.0 {
                if x == self.ball.0 as usize && y == self.ball.1 as usize {
                    print!("O");
                } else {
                    print!(" ");
                }
            }
            println!("|\r");
        }

        println!("+{}+\r", "-".repeat(MAP_LIMIT.0));
    }

    pub fn update(&mut self) {
        let diff = self.timer.elapsed().as_millis() - self.last_update;
        self.calc_bounce();

        let dx = (self.ball_direction.0 * 20.0) * (diff as f32 / 1000.0);
        let dy = (self.ball_direction.1 * 10.0) * (diff as f32 / 1000.0);

        self.ball.0 += dx;
        self.ball.1 += dy;

        // ...
        // ...
        self.last_update = self.timer.elapsed().as_millis();
    }

    fn calc_bounce(&mut self) {
        let (bx, by) = self.ball;

        if bx <= 0.0 || bx >= (MAP_LIMIT.0 as f32) {
            self.ball_direction.0 = -self.ball_direction.0;
        }

        if by <= 0.0 || by >= (MAP_LIMIT.1 as f32) {
            self.ball_direction.1 = -self.ball_direction.1;
        }
    }
}
