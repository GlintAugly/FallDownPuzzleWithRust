//! ゲームの操作方法を定義する.

use crate::{
    gameplay::{
        block::{
            block_datas::{self, BlockType}, control_block::ControlBlock, hold_block::HoldBlock, next_blocks::NextBlocks
        },
        field::{self, Field},
        key_input::{KeyCode, KeyInput},    
    }, 
    utility::{
        vector_util,
        grid::Grid,
    },
};
use std::time::Duration;
use std::sync::{Arc, Mutex};

const AUTO_REPEAT_INITIAL_DELAY_MS: u64 = 300;
const AUTO_HORIZONTAL_REPEAT_INTERVAL_MS: u64 = 500 / field::FIELD_WIDTH as u64;
const DROP_VELOCITY_MULTIPLIER: u128 = 20;
pub const HARD_DROP_MOVE_COUNT: i32 = 999;
pub const HOLD_USING: i32 = -1;

/// ゲームをコントロールするトレイト
pub trait PlayController {
    fn plan(&mut self, _: &BlockType, _: &BlockType, _: &Field) { }
    fn control(&mut self, target: &mut ControlBlock, hold_block: &mut HoldBlock, field: &Field, next_blocks: &mut NextBlocks, drop_time_ms: u128, auto_drop_count: u32) -> i32;
    fn is_pause_requested(&self) -> bool;
    fn is_player_exists(&self) -> bool {
        true
    }
}

/// プレイヤーが操作する場合に使用する構造体.
pub struct PlayerController {
    left_key: KeyCode,
    right_key: KeyCode,
    down_key: KeyCode,
    rotate_key: KeyCode,
    counter_rotate_key: KeyCode,
    hard_drop_key: KeyCode,
    hold_key: KeyCode,
    pause_key: KeyCode,
    repeat_counter_left: u32,
    repeat_counter_right: u32,
    repeat_counter_down: u32,
    key_input:  Arc<Mutex<dyn KeyInput + Send>>,
}

impl PlayerController {
    /// 新規インスタンス作成.
    pub fn new(key_input: Arc<Mutex<dyn KeyInput + Send>>) -> Self {
        PlayerController {
            left_key: KeyCode::Left,
            right_key: KeyCode::Right,
            down_key: KeyCode::Down,
            rotate_key: KeyCode::Up,
            counter_rotate_key: KeyCode::Char('z'),
            hard_drop_key: KeyCode::Char(' '),
            hold_key: KeyCode::Char('c'),
            pause_key: KeyCode::Esc,
            repeat_counter_left: 0,
            repeat_counter_right: 0,
            repeat_counter_down: 0,
            key_input: key_input,
        }
    }
}

impl PlayController for PlayerController {
    /// [ControlBlock]を操作する.
    /// 操作を加えたことで変更された場合に、変更回数を返す.
    /// ただし、ハードドロップが行われた場合には充分大きい値が返る.
    fn control(&mut self, target: &mut ControlBlock, hold_block: &mut HoldBlock, field: &Field, next_blocks: &mut NextBlocks, drop_time_ms: u128, auto_drop_count: u32) -> i32 {
        let mut move_count = 0;
        let (left_down, left_press, right_down, right_press, 
            down_press, rotate_down, counter_rotate_down, 
            hard_drop_down, hold_down) = {
            let key_input = self.key_input.lock().unwrap();
            (key_input.is_down(&self.left_key), key_input.is_press(&self.left_key), key_input.is_down(&self.right_key), key_input.is_press(&self.right_key), 
                key_input.is_press(&self.down_key), key_input.is_down(&self.rotate_key), key_input.is_down(&self.counter_rotate_key), 
                key_input.is_down(&self.hard_drop_key), key_input.is_down(&self.hold_key))
        };
        if left_down {
            if target.left(field) {
                move_count += 1;
            };
        }
        if left_press {
            let press_time = {
                let key_input = self.key_input.lock().unwrap();
                key_input.calc_elapsed(&self.left_key)
            };
            if press_time > Duration::from_millis(AUTO_REPEAT_INITIAL_DELAY_MS) {
                let repeat_count = (press_time.as_millis() - AUTO_REPEAT_INITIAL_DELAY_MS as u128)
                    / AUTO_HORIZONTAL_REPEAT_INTERVAL_MS as u128 - self.repeat_counter_left as u128;
                for _ in 0..repeat_count {
                    self.repeat_counter_left += 1;
                    if target.left(field) {
                        move_count += 1;
                    };
                }
            }
        }
        else{
            self.repeat_counter_left = 0;
        }
        if right_down {
            if target.right(field) {
                move_count += 1;
            };
        }
        if right_press {
            let press_time = {
                let key_input = self.key_input.lock().unwrap();
                key_input.calc_elapsed(&self.right_key)
            };
            if press_time > Duration::from_millis(AUTO_REPEAT_INITIAL_DELAY_MS) {
                let repeat_count = (press_time.as_millis() - AUTO_REPEAT_INITIAL_DELAY_MS as u128)
                    / AUTO_HORIZONTAL_REPEAT_INTERVAL_MS as u128 - self.repeat_counter_right as u128;
                for _ in 0..repeat_count {
                    self.repeat_counter_right += 1;
                    if target.right(field) {
                        move_count += 1;
                    };
                }
            }
        }
        else{
            self.repeat_counter_right = 0;
        }
        if down_press {
            let press_time = {
                let key_input = self.key_input.lock().unwrap();
                key_input.calc_elapsed(&self.down_key)
            };
            let mut repeat_count = (press_time.as_millis() * DROP_VELOCITY_MULTIPLIER / drop_time_ms) as u32 - self.repeat_counter_down;
            if repeat_count > 0 {
                repeat_count -=  auto_drop_count;
            }
            for _ in 0..repeat_count {
                self.repeat_counter_down += 1;
                target.down(field);
            }
        }
        else{
            self.repeat_counter_down = 0;
        }
        if rotate_down {
            if target.rotate(field) {
                move_count += 1;
            }
        }
        if counter_rotate_down {
            if target.counter_rotate(field) {
                move_count += 1;
            }
        }
        if hard_drop_down {
            target.hard_drop(field);
            move_count += HARD_DROP_MOVE_COUNT;
        }
        if hold_down {
            move_count = apply_hold(target, hold_block, next_blocks);
        }
        move_count
    }

    /// ポーズ操作が行われたかどうかを返す.
    fn is_pause_requested(&self) -> bool {
        let key_input = self.key_input.lock().unwrap();
        key_input.is_press(&self.pause_key)
    }
}

const DEFAULT_COMPUTER_MOVE_COUNT: usize = 10;
pub struct ComputerController {
    target_pos_x: i32,
    rotate_count: usize,
    use_hold: bool,
    move_wait_counter: usize,
}

impl ComputerController {
    pub fn new() -> Self {
        ComputerController {
            target_pos_x: 0,
            rotate_count: 0,
            use_hold: false,
            move_wait_counter: DEFAULT_COMPUTER_MOVE_COUNT,
        }
    }
}

impl PlayController for ComputerController {
    fn plan(&mut self, target_block_type: &BlockType, hold_block_type: &BlockType, field: &Field) {
        // とりあえず、なるべく下に配置出来るような形で組む.
        let mut max_y = 0;

        for use_hold in [false, true].iter() {
            let block_type = if *use_hold {hold_block_type} else {target_block_type};
            for rotate_count in 0..4 {
                let mut block_shape = block_datas::block_shape(*block_type);
                for _ in 0..rotate_count {
                    block_shape = vector_util::rotate_vec_90_clockwise(&block_shape);
                }
                if let Some(row) = field.get_all_grid_data().get(0) {
                    let width = row.len() as i32;
                    for target_pos_x in 0..width {
                        let now_position = Grid::new(target_pos_x, 0);
                        let ghost_grid = field.get_ghost_position(&block_shape, &now_position);
                        if ghost_grid.y > max_y {
                            // プラン更新.
                            self.target_pos_x = target_pos_x;
                            self.rotate_count = rotate_count;
                            self.use_hold = *use_hold;
                            max_y = ghost_grid.y;
                        }
                    }
                }
            }
        }
        self.move_wait_counter = DEFAULT_COMPUTER_MOVE_COUNT;
    }

    fn control(&mut self, target: &mut ControlBlock, hold_block: &mut HoldBlock, field: &Field, next_blocks: &mut NextBlocks, _: u128, _: u32) -> i32 {
        // 1フレに1回動くと不公平感があるので、待ちを入れる.
        if self.move_wait_counter > 0 {
            self.move_wait_counter -= 1;
            return 0;
        }

        // planに応じて動かす.1回の移動で複数のコマンドが打たれないように注意.
        let mut move_count = 0;
        if self.use_hold {
            move_count = apply_hold(target, hold_block, next_blocks);
            self.use_hold = false;
        }
        else if self.rotate_count > 0 {
            target.rotate(field);
            self.rotate_count -= 1;
            move_count += 1;
        }
        else if target.position.x < self.target_pos_x {
            target.right(field);
            move_count += 1;
        }
        else if target.position.x > self.target_pos_x {
            target.left(field);
            move_count += 1;
        }
        // 移動する必要がなければハードドロップする.
        if move_count == 0 {
            target.hard_drop(field);
            move_count += HARD_DROP_MOVE_COUNT;
        }
        self.move_wait_counter = DEFAULT_COMPUTER_MOVE_COUNT;
        move_count
    }

    /// コンピューターはポーズ要求しない.
    fn is_pause_requested(&self) -> bool {
        false
    }

    fn is_player_exists(&self) -> bool {
        false
    }
}


fn apply_hold(target: &mut ControlBlock, hold_block: &mut HoldBlock, next_blocks: &mut NextBlocks) -> i32 {
    if hold_block.can_hold() {
        let current_block_type = target.block_type;
        if let Some(held_block_type) = hold_block.hold(current_block_type) {
            if held_block_type != BlockType::None {
                target.apply_block(held_block_type, block_datas::BLOCK_START_POSITION); // スタート位置にリセット
            }
            else{
                // 新しいブロックを生成
                let next_block = next_blocks.next();
                target.apply_block(next_block, block_datas::BLOCK_START_POSITION);
            }
        }
        return HOLD_USING;
    }
    0
}