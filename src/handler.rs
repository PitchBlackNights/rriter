use crate::app::{App, AppResult};
use crate::text_input::handler::handle_key;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match (key_event.code, key_event.modifiers) {
        // Exit application on `CTRL+[q]x2`
        (KeyCode::Char('q'), KeyModifiers::CONTROL)
        | (KeyCode::Char('Q'), KeyModifiers::CONTROL) => {
            if app.last_char == key_event.code
                && app.char_ticks_left != 0
                && app.rep_char_cooldown == 0
            {
                app.quit()
            } else {
                app.reset_rep_char(3, key_event.code)
            }
        }
        // Other handlers you could add here.
        (_, _) => handle_key(key_event)
    }
    app.last_char = KeyCode::from(key_event.code);
    Ok(())
}
