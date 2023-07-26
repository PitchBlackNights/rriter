use std::error;
use crossterm::event::KeyCode;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Tick left for repeated character to be used
    pub char_ticks_left: u8,
    /// Cooldown time before a character is counted as repeated
    pub rep_char_cooldown: u8,
    /// Last character that was used
    pub last_char: KeyCode,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            char_ticks_left: 0,
            rep_char_cooldown: 0,
            last_char: KeyCode::Char('\0'),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        if let Some(res) = self.char_ticks_left.checked_sub(1) {
            self.char_ticks_left = res;
        }
        if let Some(res) = self.rep_char_cooldown.checked_sub(1) {
            self.rep_char_cooldown = res;
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Reset the repeated character system with the given ticks and character
    pub fn reset_rep_char(&mut self, ticks: u8, key_code: KeyCode) {
        self.char_ticks_left = ticks;
        self.rep_char_cooldown = 1;
        self.last_char = key_code;
    }
}
