//! キー入力操作.

use std::{time::{Duration, Instant}, collections::HashMap, io};
use crate::gameplay::key_input::{KeyType, KeyInput};
use crossterm::{self, event::{self, Event, KeyEventKind}};
use crossterm::event::KeyCode as ConsoleKeyCode;

/// キー入力を管理する構造体.
pub struct ConsoleKeyInput {
    last_downed: HashMap<ConsoleKeyCode, Instant>,
    before_downed: HashMap<ConsoleKeyCode, bool>,
    down: HashMap<ConsoleKeyCode, bool>,
}

impl ConsoleKeyInput {
    /// 新規インスタンス作成
    pub fn new() -> Self {
        Self {
            last_downed: HashMap::new(),
            before_downed: HashMap::new(),
            down: HashMap::new(),
        }
    }
    fn key_code_to_console_key_code(key: &KeyType) -> ConsoleKeyCode {
        match key {
            KeyType::MenuDecide => ConsoleKeyCode::Enter,
            KeyType::MenuSelectUp => ConsoleKeyCode::Up,
            KeyType::MenuSelectDown => ConsoleKeyCode::Down,
            KeyType::MenuSelectLeft => ConsoleKeyCode::Left,
            KeyType::MenuSelectRight => ConsoleKeyCode::Right,
            KeyType::P1Up => ConsoleKeyCode::Char('w'),
            KeyType::P1Down => ConsoleKeyCode::Char('s'),
            KeyType::P1Left => ConsoleKeyCode::Char('a'),
            KeyType::P1Right => ConsoleKeyCode::Char('d'),
            KeyType::P1Rotate => ConsoleKeyCode::Char('x'),
            KeyType::P1CounterRotate => ConsoleKeyCode::Char('z'),
            KeyType::P1HardDrop => ConsoleKeyCode::Char('f'),
            KeyType::P1Hold => ConsoleKeyCode::Char('c'),
            KeyType::P1Pause => ConsoleKeyCode::Char('r'),
            KeyType::P2Up => ConsoleKeyCode::Char('i'),
            KeyType::P2Down => ConsoleKeyCode::Char('k'),
            KeyType::P2Left => ConsoleKeyCode::Char('j'),
            KeyType::P2Right => ConsoleKeyCode::Char('l'),
            KeyType::P2Rotate => ConsoleKeyCode::Char(','),
            KeyType::P2CounterRotate => ConsoleKeyCode::Char('m'),
            KeyType::P2HardDrop => ConsoleKeyCode::Char(';'),
            KeyType::P2Hold => ConsoleKeyCode::Char('.'),
            KeyType::P2Pause => ConsoleKeyCode::Char('p'),
        }
    }
}

impl KeyInput for ConsoleKeyInput {
    /// 非ブロッキングにキーイベントを取得してフラグをセットする
    fn poll_input(&mut self) -> io::Result<()> {
        // downの状況をムーブ.
        self.before_downed = self.down.clone();
        self.down = HashMap::new();
        // すぐに返るポーリング（0ms）
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_ev) = event::read()? {
                let now = Instant::now();
                if let KeyEventKind::Press = key_ev.kind {
                    self.down.insert(key_ev.code, true);
                    if let None = self.before_downed.get(&key_ev.code) {
                        self.last_downed.insert(key_ev.code, now);
                    }
                }
            }
        }
        Ok(())
    }

    /// 指定したキーが押されているか
    fn is_press(&self, key: &KeyType) -> bool {
        let key: ConsoleKeyCode = ConsoleKeyInput::key_code_to_console_key_code(key);
        *self.down.get(&key).unwrap_or(&false)
    }
    /// 指定したキーが押された瞬間か
    fn is_down(&self, key: &KeyType) -> bool {
        let key: ConsoleKeyCode = ConsoleKeyInput::key_code_to_console_key_code(key);
        *self.down.get(&key).unwrap_or(&false) && !*self.before_downed.get(&key).unwrap_or(&false)
    }
    /// 指定したキーが離された瞬間か
    fn is_up(&self, key: &KeyType) -> bool {
        let key: ConsoleKeyCode = ConsoleKeyInput::key_code_to_console_key_code(key);
        !*self.down.get(&key).unwrap_or(&false) && *self.before_downed.get(&key).unwrap_or(&false)
    }
    /// 指定したキーが最後に押されてからの経過時間を取得
    fn calc_elapsed(&self, key: &KeyType) -> Duration {
        let key: ConsoleKeyCode = ConsoleKeyInput::key_code_to_console_key_code(key);
        if let Some(press) = self.down.get(&key) {
            if !press {
                return Duration::from_secs(0);
            }
        } else {
            return Duration::from_secs(0);
        }
        self.last_downed.get(&key).map_or(Duration::from_secs(0), 
                                        |&instant| instant.elapsed())
    }
}
