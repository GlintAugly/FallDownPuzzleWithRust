//! ゲーム全体のマネージャー.
use crate::gameplay::{
    gameplay_manager::{GameplayManager, PlayerType},
    game_renderer_sender::GameRendererSender,
    key_input::{KeyInput, KeyType},
};
use std::sync::{Arc, Mutex};

pub enum GameState {
    Title,
    Playing,
    Paused,
    GameOver,
}

#[derive(PartialEq)]
pub enum TitleChoice {
    Play,
    Exit,
}

pub enum PlayStyle {
    Solo,
    WithNPC(usize),
    VSPlayer,
}

/// ゲーム全体を管理する構造体.
/// 基本的にはシングルトンを想定している.
pub struct GameManager {
    state: GameState,
    title_choice_command: TitleChoice,
    play_style: PlayStyle,
    high_score: u64,
    level: u32,
    pub gameplay_managers: Vec<GameplayManager>,
    high_score_updated: bool,
    renderer_sender: Box<dyn GameRendererSender + Send>,
    key_input_manager: Arc<Mutex<dyn KeyInput + Send>>,
}

impl GameManager {
    /// 新規インスタンス作成.
    pub fn new(renderer_sender: Box<dyn GameRendererSender + Send>, key_input_manager: Arc<Mutex<dyn KeyInput + Send>>) -> Self {
        GameManager {
            state: GameState::Title,
            title_choice_command: TitleChoice::Play,
            play_style: PlayStyle::Solo,
            high_score: 0,
            level: 1,
            gameplay_managers: vec![],
            high_score_updated: false,
            renderer_sender,
            key_input_manager: key_input_manager,
        }
    }

    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    pub fn get_title_choice_command(&self) -> &TitleChoice {
        &self.title_choice_command
    }

    pub fn get_play_style(&self) -> &PlayStyle {
        &self.play_style
    }

    pub fn get_high_score_updated(&self) -> bool {
        self.high_score_updated
    }

    pub fn get_high_score(&self) -> u64 {
        self.high_score
    }

    /// 有人プレイヤーでインゲームを作成する.
    pub fn create_player(&mut self, player_type: PlayerType) {
        self.gameplay_managers.push(GameplayManager::with_player_controller(self.level, player_type, self.key_input_manager.clone()));
    }

    /// npcプレイヤーでインゲームを作成する.
    pub fn create_npc(&mut self) {
        self.gameplay_managers.push(GameplayManager::with_npc_controller(self.level));
    }

    /// 更新処理.
    pub fn update(&mut self) -> bool{
        let _ = self.key_input_manager.lock().unwrap().poll_input();
        match self.state {
            GameState::Title => {
                // タイトル画面の更新処理
                let (press_select_up, press_select_down, 
                    press_select_left, press_select_right, 
                    press_decide) = {
                    let key_input = self.key_input_manager.lock().unwrap();
                    (key_input.is_down(&KeyType::MenuSelectUp), key_input.is_down(&KeyType::MenuSelectDown), 
                        key_input.is_down(&KeyType::MenuSelectLeft), key_input.is_down(&KeyType::MenuSelectRight), 
                        key_input.is_down(&KeyType::MenuDecide))
                };
                if press_select_up || press_select_down {
                    self.title_choice_command = match self.title_choice_command {
                        TitleChoice::Play => TitleChoice::Exit,
                        TitleChoice::Exit => TitleChoice::Play,
                    }
                }
                if press_select_right {
                    if self.title_choice_command == TitleChoice::Play {
                        self.play_style = match self.play_style {
                            PlayStyle::Solo => PlayStyle::WithNPC(1),
                            PlayStyle::WithNPC(_) => PlayStyle::VSPlayer,
                            PlayStyle::VSPlayer => PlayStyle::Solo,
                        }
                    }
                }
                if press_select_left {
                    if self.title_choice_command == TitleChoice::Play {
                        self.play_style = match self.play_style {
                            PlayStyle::Solo => PlayStyle::VSPlayer,
                            PlayStyle::WithNPC(_) => PlayStyle::Solo,
                            PlayStyle::VSPlayer => PlayStyle::WithNPC(1),
                        }
                    }
                }
                if press_decide {
                    match self.title_choice_command {
                        TitleChoice::Play => {
                            self.state = GameState::Playing;
                            match self.play_style {
                                PlayStyle::Solo => self.create_player(PlayerType::Player1),
                                PlayStyle::WithNPC(npc_count) => {
                                    self.create_player(PlayerType::Player1);
                                    for _ in 0..npc_count{
                                        self.create_npc();
                                    }
                                },
                                PlayStyle::VSPlayer => {
                                    self.create_player(PlayerType::Player1);
                                    self.create_player(PlayerType::Player2);
                                },
                            }
                            
                            self.high_score_updated = false;
                        },
                        TitleChoice::Exit => return false,
                    };
                }
            }
            GameState::Playing => {
                for gameplay_manager in &mut self.gameplay_managers {
                    gameplay_manager.update();
                    if gameplay_manager.pause_requested() {
                        self.state = GameState::Paused;
                    }
                }
                // 攻撃受け入れ
                for i in 0..self.gameplay_managers.len() {
                    let attack_power = self.gameplay_managers[i].pop_attack_power();
                    if attack_power > 0 {
                        for j in 0..self.gameplay_managers.len() {
                            if i != j {
                                self.gameplay_managers[j].apply_attack(attack_power);
                            }
                        }
                    }
                }
                if self.gameplay_managers.iter().all(|gm| gm.is_game_over()) {
                    self.state = GameState::GameOver;
                }
            }
            GameState::Paused => {
                // ポーズ中の更新処理
                for gameplay_manager in &mut self.gameplay_managers {
                    if gameplay_manager.pause_requested() {
                        self.state = GameState::Playing;
                    }
                }
            }
            GameState::GameOver => {
                // ゲームオーバー時の更新処理
                let max_score = self.gameplay_managers.iter().map(|gm| gm.get_score()).max().unwrap_or(0);
                if max_score > self.high_score {
                    self.high_score_updated = true;
                    self.high_score = max_score;
                }
                let press_enter = {
                    let key_input = self.key_input_manager.lock().unwrap();
                    key_input.is_down(&KeyType::MenuDecide)
                };
                if press_enter {
                    self.gameplay_managers.clear();
                    self.state = GameState::Title;
                }
            }
        }
        self.renderer_sender.game_sender(&self);
        true
    }
}
