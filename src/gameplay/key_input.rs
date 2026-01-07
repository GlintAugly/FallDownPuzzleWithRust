//! キー入力のトレイト

use std::time::Duration;
use std::io;
pub enum KeyType {
    MenuDecide,
    MenuSelectUp,
    MenuSelectDown,
    MenuSelectLeft,
    MenuSelectRight,
    P1Up,
    P1Down,
    P1Left,
    P1Right,
    P1Rotate,
    P1CounterRotate,
    P1HardDrop,
    P1Hold,
    P1Pause,
    P2Up,
    P2Down,
    P2Left,
    P2Right,
    P2Rotate,
    P2CounterRotate,
    P2HardDrop,
    P2Hold,
    P2Pause,
}

pub trait KeyInput{
    fn poll_input(&mut self) -> io::Result<()>;
    fn is_press(&self, key: &KeyType) -> bool;
    fn is_down(&self, key: &KeyType) -> bool;
    fn is_up(&self, key: &KeyType) -> bool;
    fn calc_elapsed(&self, key: &KeyType) -> Duration;
}