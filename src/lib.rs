mod console_key_input;
mod gameplay;
mod console_renderer;
mod console_renderer_sender;
mod utility;

use crate::{
    console_key_input::ConsoleKeyInput,
    gameplay::game_manager::GameManager,
    console_renderer::render_manager::RenderManager,
    console_renderer_sender::game_sender::GameSender,
};
use std::{thread, time::{Instant, Duration}, sync::Mutex};
use std::sync::{Arc};
use once_cell::sync::Lazy;

const FPS: u64 = 20;
const FRAME_TIME_MILLIS: u64 = 1000 / FPS;

pub static GAME_MANAGER: Lazy<Mutex<GameManager>> = Lazy::new(|| {
    Mutex::new(GameManager::new(Box::new(GameSender::new())
                                , Arc::new(Mutex::new(ConsoleKeyInput::new()))))
});

pub static KEY_INPUT: Lazy<Mutex<ConsoleKeyInput>> = Lazy::new(|| {
    Mutex::new(ConsoleKeyInput::new())
});

pub static RENDER_MANAGER: Lazy<Mutex<RenderManager>> = Lazy::new(|| {
    Mutex::new(RenderManager::new())
});

/// ゲームロジックの更新.
fn update() -> bool{
    // ゲーム状態に応じた更新.
    if !GAME_MANAGER.lock().unwrap().update() {
        // ゲーム終了処理.
        return false;
    }
    true
}

/// ゲーム描画の更新.
fn render() {
    RENDER_MANAGER.lock().unwrap().clear();
    RENDER_MANAGER.lock().unwrap().render();
}

/// メインループ.
pub fn main_loop() {
    loop {
        let last_update = Instant::now();
        if !update() {
            break;
        }
        render();
        let now = Instant::now();
        if Duration::from_millis(FRAME_TIME_MILLIS) > now.duration_since(last_update) {
            let wait_time = Duration::from_millis(FRAME_TIME_MILLIS) - now.duration_since(last_update);
            thread::sleep(wait_time);
        }
    }
}