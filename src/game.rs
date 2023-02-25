use std::{ops::RangeInclusive, time::Instant};

const MAP_LIMIT: (usize, usize) = (40, 20);

pub struct GameData {
    pub state: GameState,

    pub ball: (f32, f32),
    ball_direction: (f32, f32),

    pub player1: PlayerData,
    pub player2: PlayerData,

    last_update: u128,
    timer: Instant,
}

pub struct PlayerData {
    pub position: f32,
    pub size: usize,
    pub range: RangeInclusive<usize>,
    pub score: u32,
}

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Lost,
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData {
            position: MAP_LIMIT.1 as f32 / 2.0,
            size: 3,
            range: 0..=0,
            score: 0,
        }
    }
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            state: GameState::Playing,

            ball: (20.0, 10.0),
            ball_direction: (1.0, -0.5),

            player1: PlayerData::new(),
            player2: PlayerData::new(),

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
                print!("{}", self.draw(x, y));
            }
            println!("|\r");
        }

        println!("+{}+\r", "-".repeat(MAP_LIMIT.0));
    }

    fn draw(&self, x: usize, y: usize) -> char {
        if x == 0 && self.player1.range.contains(&y) {
            return '#';
        }
        if x == MAP_LIMIT.0 - 1 && self.player2.range.contains(&y) {
            return '@';
        }

        if x == self.ball.0 as usize && y == self.ball.1 as usize {
            return 'O';
        }

        return ' ';
    }

    pub fn update(&mut self) {
        let diff = self.timer.elapsed().as_millis() - self.last_update;
        let dx = (self.ball_direction.0 * 20.0) * (diff as f32 / 1000.0);
        let dy = (self.ball_direction.1 * 10.0) * (diff as f32 / 1000.0);

        self.ball.0 += dx;
        self.ball.1 += dy;

        self.calc_bounce();
        self.check_state();
        self.calc_player_range();

        self.last_update = self.timer.elapsed().as_millis();
    }

    fn calc_player_range(&mut self) {
        let p1_range = self.player1.position as usize - self.player1.size
            ..=self.player1.position as usize + self.player1.size;

        let p2_range = self.player2.position as usize - self.player2.size
            ..=self.player2.position as usize + self.player2.size;

        self.player1.range = p1_range;
        self.player2.range = p2_range;
    }

    fn calc_bounce(&mut self) {
        let (bx, by) = self.ball;

        if bx <= 1.0 && self.player1.range.contains(&(by as usize))
            || bx >= (MAP_LIMIT.0 - 2) as f32 && self.player2.range.contains(&(by as usize))
        {
            self.ball_direction.0 = -self.ball_direction.0;
        }

        if by < 0.0 || by > (MAP_LIMIT.1 as f32) {
            self.ball_direction.1 = -self.ball_direction.1;
        }
    }

    fn check_state(&mut self) {
        let (bx, _) = self.ball;

        if bx <= 0.0 || bx >= (MAP_LIMIT.0 as f32) {
            self.state = GameState::Lost;
        }
    }
}
