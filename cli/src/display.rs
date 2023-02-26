use anyhow::Result;
use crossterm::{
    event::{self, KeyCode},
    execute, terminal,
};

pub struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode");
        execute!(std::io::stdout(), terminal::LeaveAlternateScreen)
            .expect("Unable to leave alternate screen");
    }
}

impl CleanUp {
    pub fn new() -> Result<Self> {
        terminal::enable_raw_mode()?;
        execute!(
            std::io::stdout(),
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All)
        )?;

        return Ok(CleanUp);
    }
}

pub fn catch_input(millis: u64) -> Result<KeyCode> {
    if event::poll(std::time::Duration::from_millis(millis))? {
        if let event::Event::Key(key) = event::read()? {
            return Ok(key.code);
        }
    }

    return Ok(KeyCode::Null);
}

pub fn clear_console() {
    execute!(
        std::io::stdout(),
        crossterm::cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )
    .expect("Error");
}

pub fn move_zero() {
    execute!(std::io::stdout(), crossterm::cursor::MoveTo(0, 0),).expect("Error");
}
