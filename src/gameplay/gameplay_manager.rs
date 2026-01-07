//! インゲームのマネージャー.
//! 複数呼び出すことで、各マネージャーごとにプレイヤーを設定出来る(ようになるかも.)

use crate::gameplay::{
    block::{
        block_datas::{self, BlockType}, control_block::ControlBlock, hold_block::HoldBlock, next_blocks::NextBlocks
    }, 
    controller::{self, ComputerController, PlayController, PlayerKeyAssigns,PlayerController}, 
    field::Field, key_input::KeyInput, 
    score_calculator::{AttackPowerCalculator, ScoreCalculator, SimpleAttackPowerCalculator, SimpleScoreCalculator}, 
    t_spin_checker::{TSpinChecker, TSpinType}
};
use crate::utility::grid::Grid;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};

const DEFAULT_DROP_SPEED_MS: u32 = 1000;
const LOCK_DOWN_TIME_MS: u32 = 500;
const LOCK_DOWN_COUNT_MAX: u32 = 15;
const MAX_ERACE_LINES: u32 = 3;
const ERACE_LINE_WAIT_MILLIS: u128 = 500;
const DROP_LINE_WAIT_MILLIS: u128 = 500;
const BEFORE_CONTROLLING_WAIT_MILLIS: u128 = 3000;
enum PlayState {
    WaitStart,
    Waiting,
    StartControlling,
    Controlling,
    Dropped,
    Eracing,
    Dropping,
}

pub enum PlayerType {
    Player1,
    Player2,
    NPC,
}

/// ゲームのステータス.
pub struct GameplayStats {
    pub level: u32,
    pub erace_lines: u32,
    pub t_spin_erace_lines: u32,
    pub combos: u32,
    pub max_erace_count: u32,
}
/// インゲームを管理・運営していく構造体.
pub struct GameplayManager {
    field: Field,
    next_blocks: NextBlocks,
    hold_block: HoldBlock,
    control_block: ControlBlock,
    is_game_over: bool,
    controller: Box<dyn PlayController + Send>,
    score_calculator: Box<dyn ScoreCalculator + Send>,
    attack_power_calculator: Box<dyn AttackPowerCalculator + Send>,
    attack_power: usize,
    applied_attack: usize,
    score: u64,
    stats: GameplayStats,
    state: PlayState,
    wait_timer: Instant,
    t_spin_checker: TSpinChecker,
    t_spin_mode: TSpinType,
    combo_mode: bool,
    drop_speed: u32,
    move_counter: u32,
    lock_down_timer: Instant,
    lock_down_lowest_height: i32,
    last_drop_time: Instant,
}

impl GameplayManager {
    /// 有人プレイヤーでの新規インスタンス作成.
    pub fn with_player_controller(level: u32, player_type: PlayerType, key_input: Arc<Mutex<dyn KeyInput + Send>>) -> Self {
        let key_assigns = match player_type {
            PlayerType::Player1 => PlayerKeyAssigns::player1_keys(),
            PlayerType::Player2 => PlayerKeyAssigns::player2_keys(),
            PlayerType::NPC => panic!("NPC cannot use player controller."),
        };
        GameplayManager::new(level, Box::new(PlayerController::new(key_assigns, key_input)))
    }

    // NPCでの新規インスタンス作成.
    pub fn with_npc_controller(level: u32) -> Self {
        GameplayManager::new(level, Box::new(ComputerController::new()))
    }

    /// 新規インスタンス作成.操作するためのインスタンスが必要.
    pub fn new(level: u32, controller: Box<dyn PlayController + Send>) -> Self {
        GameplayManager {
            field: Field::new(),
            next_blocks: NextBlocks::new(),
            hold_block: HoldBlock::new(),
            control_block: ControlBlock::new(),
            is_game_over: false,
            controller: controller,
            score_calculator: Box::new(SimpleScoreCalculator::new()),
            attack_power_calculator: Box::new(SimpleAttackPowerCalculator::new()),
            attack_power: 0,
            applied_attack: 0,
            score: 0,
            stats: GameplayStats {
                erace_lines: 0,
                t_spin_erace_lines: 0,
                combos: 0,
                max_erace_count: 0,
                level: level,
            },
            state: PlayState::WaitStart,
            wait_timer: Instant::now(),
            t_spin_checker: TSpinChecker::new(),
            t_spin_mode: TSpinType::None,
            combo_mode: false,
            drop_speed: DEFAULT_DROP_SPEED_MS,
            move_counter: 0,
            lock_down_timer: Instant::now(),
            lock_down_lowest_height: 0,
            last_drop_time: Instant::now(),
        }
    }
    
    /// インゲームの更新処理.
    pub fn update(&mut self) {
        if self.is_game_over {
            return;
        }
        let now = Instant::now();
        match self.state {
            PlayState::WaitStart => {
                // 待機処理前にしたいことをする.
                // 攻撃を受けていたらここで受け入れる.
                if self.applied_attack > 0 {
                    self.field.apply_attack(self.applied_attack);
                    self.applied_attack = 0;
                }
                self.state = PlayState::Waiting;
            }
            PlayState::Waiting => {
                // 次のブロックが来るまでの待機処理
                if self.wait_timer.elapsed().as_millis() <= BEFORE_CONTROLLING_WAIT_MILLIS {
                    self.state = PlayState::StartControlling;
                }
            }
            PlayState::StartControlling => {
                // ゲームオーバーのチェック
                if self.field.check_game_over(&self.next_blocks.show_next_block(0)) {
                    self.is_game_over = true;
                    return;
                }
                // 状態のリセット
                self.hold_block.allow_hold();
                self.t_spin_mode = TSpinType::None;
                self.move_counter = 0;
                self.lock_down_timer = Instant::now();
                self.lock_down_lowest_height = 0;
                self.last_drop_time = Instant::now();
                // ブロックの配置.
                self.control_block.apply_block(self.next_blocks.next(), block_datas::BLOCK_START_POSITION);
                self.state = PlayState::Controlling;
                // 操作プランの策定.
                let next_hold_block = if self.hold_block.get_holding_block() == BlockType::None {self.next_blocks.show_next_block(0)} else {self.hold_block.get_holding_block()};
                self.controller.plan(&self.control_block.block_type, &next_hold_block, &self.field);
            }
            PlayState::Controlling => {
                // 操作可能状態での処理.
                // 自動落下処理
                let now = Instant::now();
                let down_count = now.duration_since(self.last_drop_time).as_millis() as u32 / self.drop_speed;
                for _ in 0..down_count {
                    self.control_block.down(&self.field);
                }
                if down_count > 0 {
                    let remain = Duration::from_millis(now.duration_since(self.last_drop_time).as_millis() as u64 % self.drop_speed as u64);
                    self.last_drop_time = now - remain;
                    self.lock_down_timer = Instant::now();
                    self.t_spin_mode = TSpinType::None;
                }
                // プレイヤー操作処理
                self.t_spin_checker.set_block_data(&self.control_block);
                let move_count = self.controller.control(&mut self.control_block, &mut self.hold_block, &self.field, &mut self.next_blocks, self.drop_speed as u128, down_count);
                if move_count == controller::HOLD_USING {
                    //ホールドされたのでロックダウン周りはリセット.
                    self.move_counter = 0;
                    self.lock_down_timer = Instant::now();
                }
                else if move_count > 0 {
                    self.move_counter += move_count as u32;
                    self.lock_down_timer = Instant::now();
                    
                    self.t_spin_mode = if self.t_spin_checker.check_t_spinned(&self.control_block){
                            // Tブロックが回転しているのでTスピン判定
                            // フィールド情報の一部抜粋.
                            let mut t_block_field = vec![vec![BlockType::None; 3]; 3];
                            for x in 0..3{
                                for y in 0..3{
                                    let check_pos = Grid::new(self.control_block.position.x + x as i32, self.control_block.position.y - y as i32);
                                    t_block_field[y][x] = if !self.field.check_position_in_field(&check_pos){
                                            // とりあえずNone以外ならなんでもいい.
                                            BlockType::I
                                        }
                                        else{
                                            self.field.get_grid_data(&check_pos)
                                        };
                                }
                            }
                            self.t_spin_checker.calc_t_spin_type(&self.control_block, &t_block_field)
                        }
                        else {
                            TSpinType::None
                        };
                }
                // ハードドロップでなく、下限値更新していたら移動回数をリセット.
                // lowestだけど、下向きに正のため大きい方が下に来る.
                if self.move_counter < controller::HARD_DROP_MOVE_COUNT as u32 && self.control_block.position.y > self.lock_down_lowest_height {
                    self.lock_down_lowest_height = self.control_block.position.y;
                    self.move_counter = 0;
                }
                // ロックダウン判定
                if self.field.check_collision(&self.control_block.block, &Grid::new(self.control_block.position.x, self.control_block.position.y + 1)) {
                    if self.move_counter >= LOCK_DOWN_COUNT_MAX || now.duration_since(self.lock_down_timer).as_millis() as u32 >= LOCK_DOWN_TIME_MS {
                        self.field.lock_block(&self.control_block.block, &self.control_block.position);
                        self.control_block.delete_block();
                        self.state = PlayState::Dropped;
                    }
                }
            }
            PlayState::Dropped => {
                // ラインクリアのチェック.
                let eraced_lines = self.field.clear_lines();
                if eraced_lines > 0 {
                    self.stats.erace_lines += eraced_lines;
                    if eraced_lines == MAX_ERACE_LINES {
                        self.stats.max_erace_count += 1;
                    }
                    if self.t_spin_mode != TSpinType::None {
                        self.stats.t_spin_erace_lines += eraced_lines;
                    }
                    self.score += self.score_calculator.calc(eraced_lines, self.t_spin_mode, self.stats.combos); 
                    self.combo_mode = true;
                    self.stats.combos += 1;
                    self.attack_power = self.attack_power_calculator.calc(eraced_lines, self.t_spin_mode, self.stats.combos);
                    self.state = PlayState::Eracing;
                }
                else{
                    self.combo_mode = false;
                    self.stats.combos = 0;
                    self.state = PlayState::WaitStart;
                }
                self.wait_timer = now;
            }
            PlayState::Eracing => {
                // ライン消去中の処理.
                if self.wait_timer.elapsed().as_millis() >= ERACE_LINE_WAIT_MILLIS {
                    self.wait_timer = now;
                    self.state = PlayState::Dropping;
                }
            }
            PlayState::Dropping => {
                // 空白ライン埋めの処理.
                self.field.drop_lines();
                if self.wait_timer.elapsed().as_millis() >= DROP_LINE_WAIT_MILLIS {
                    self.wait_timer = now;
                    self.state = PlayState::WaitStart;
                }
            }
        }
    }

    /// 攻撃力を取り出す.1度取り出したら0にしてしまう.
    pub fn pop_attack_power(&mut self) -> usize {
        let power = self.attack_power;
        self.attack_power = 0;
        power
    }

    /// 攻撃を受け入れる.
    pub fn apply_attack(&mut self, attack_power: usize) {
        self.applied_attack += attack_power;
    }

    /// ゲームオーバーになったかどうかを返す.
    /// ゲームオーバー判定自体は更新処理で行われている.
    pub fn is_game_over(&self) -> bool {
        self.is_game_over
    }

    /// 現在のスコアを返す.
    /// 有人プレイヤーでないと0が返る.
    pub fn get_score(&self) -> u64 {
        if self.controller.is_player_exists() {
            return self.score
        }
        0
    }

    /// コントローラーからポーズが要求されているかを返す.
    pub fn pause_requested(&self) -> bool {
        self.controller.is_pause_requested()
    }

    /// コントローラーの参照を返す.
    pub fn get_control_block(&self) -> &ControlBlock {
        &self.control_block
    }

    /// 表示するフィールド情報を返す.
    pub fn get_field_data(&self) -> Vec<Vec<BlockType>> {
        self.field.get_all_grid_data()
    }

    /// 影の位置を返す.
    pub fn get_ghost_pos(&self) -> Grid {
        self.field.get_ghost_position(&self.control_block.block, &self.control_block.position)
    }

    /// 次のブロックの情報を返す.
    pub fn get_next_block(&self, look_ahead: usize) -> BlockType {
        self.next_blocks.show_next_block(look_ahead)
    }

    /// ホールドされているブロックの情報を返す.
    pub fn get_hold_block(&self) -> BlockType {
        self.hold_block.get_holding_block()
    }
    /// スコア以外のステータス文字列を返す.
    /// 幅は半角19字.
    pub fn get_stats(&self) -> &GameplayStats {
        &self.stats
    }
}