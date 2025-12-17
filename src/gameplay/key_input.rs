//! キー入力のトレイト

use std::time::Duration;
use std::io;
pub enum KeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Esc,
    Char(char)
}

pub trait KeyInput{
    fn poll_input(&mut self) -> io::Result<()>;
    fn is_press(&self, key: &KeyCode) -> bool;
    fn is_down(&self, key: &KeyCode) -> bool;
    fn is_up(&self, key: &KeyCode) -> bool;
    fn calc_elapsed(&self, key: &KeyCode) -> Duration;
}