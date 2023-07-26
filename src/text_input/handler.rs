use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyModifiers, KeyEvent};

enum InputRequest {
    SetCursor(usize),
    InsertChar(char),
    GoToPrevChar,
    GoToNextChar,
    GoToPrevWord,
    GoToNextWord,
    GoToLineStart,
    GoToLineEnd,
    DeletePrevChar,
    DeleteNextChar,
    DeletePrevWord,
    DeleteNextWord,
    DeleteSelection,
    DeleteLine,
}

pub fn handle_key(_key_event: KeyEvent) {

}
