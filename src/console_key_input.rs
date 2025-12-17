//! キー入力操作.

use std::{time::{Duration, Instant}, collections::HashMap, io};
use crate::gameplay::key_input::{KeyCode, KeyInput};
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
    fn key_code_to_console_key_code(key: &KeyCode) -> ConsoleKeyCode {
        match key {
            KeyCode::Backspace => ConsoleKeyCode::Backspace,
            KeyCode::Enter => ConsoleKeyCode::Enter,
            KeyCode::Left => ConsoleKeyCode::Left,
            KeyCode::Right => ConsoleKeyCode::Right,
            KeyCode::Up => ConsoleKeyCode::Up,
            KeyCode::Down => ConsoleKeyCode::Down,
            KeyCode::Esc => ConsoleKeyCode::Esc,
            KeyCode::Char(char) => ConsoleKeyCode::Char(*char), 
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
    fn is_press(&self, key: &KeyCode) -> bool {
        let key: ConsoleKeyCode = ConsoleKeyInput::key_code_to_console_key_code(key);
        *self.down.get(&key).unwrap_or(&false)
    }
    /// 指定したキーが押された瞬間か
    fn is_down(&self, key: &KeyCode) -> bool {
        let key: ConsoleKeyCode = ConsoleKeyInput::key_code_to_console_key_code(key);
        *self.down.get(&key).unwrap_or(&false) && !*self.before_downed.get(&key).unwrap_or(&false)
    }
    /// 指定したキーが離された瞬間か
    fn is_up(&self, key: &KeyCode) -> bool {
        let key: ConsoleKeyCode = ConsoleKeyInput::key_code_to_console_key_code(key);
        !*self.down.get(&key).unwrap_or(&false) && *self.before_downed.get(&key).unwrap_or(&false)
    }
    /// 指定したキーが最後に押されてからの経過時間を取得
    fn calc_elapsed(&self, key: &KeyCode) -> Duration {
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
