//! ブロックが配置されるフィールドを定義.

use crate::gameplay::block::block_datas::{self, BlockType};
use crate::utility::{
    grid::Grid,
    vector_util,
};
use rand::{self, Rng};
pub const FIELD_WIDTH: usize = 10;
pub const FIELD_HEIGHT_WITH_OUTSIDE: usize = block_datas::BLOCK_START_POSITION_Y as usize * 2;

/// ブロックが配置されるフィールドの構造体.
#[derive(Debug)]
pub struct Field{
    grid_data: [[BlockType; FIELD_WIDTH]; FIELD_HEIGHT_WITH_OUTSIDE],
    force_gameover: bool
}

impl Field{
    /// 新規インスタンス作成.最初はからっぽ.
    pub fn new() -> Self {
        Field {
            grid_data: [[BlockType::None; FIELD_WIDTH]; FIELD_HEIGHT_WITH_OUTSIDE],
            force_gameover: false,
        }
    }
    /// 全て埋まった行を消す.消した行数を返す.
    pub fn clear_lines(&mut self) -> u32 {
        let mut cleared_lines = 0;
        for y in (0..FIELD_HEIGHT_WITH_OUTSIDE).rev() {
            if self.grid_data[y].iter().all(|&block| block != BlockType::None) {
                self.grid_data[y] = [BlockType::None; FIELD_WIDTH];
                cleared_lines += 1;
            }
        }
        cleared_lines
    }

    /// 全て空のラインを走査して、下に落として空のラインを埋める.
    pub fn drop_lines(&mut self) {
        for y in (1..FIELD_HEIGHT_WITH_OUTSIDE).rev() {
            if self.grid_data[y].iter().all(|&block| block == BlockType::None) {
                for pull_y in (1..=y).rev() {
                    self.grid_data[pull_y] = self.grid_data[pull_y - 1];
                }
                self.grid_data[0] = [BlockType::None; FIELD_WIDTH];
            }
        }
    }

    /// フィールドがいっぱいかどうかを返す.
    pub fn check_game_over(&self, next_block: &BlockType) -> bool {
        // 他の要因でゲームオーバー扱いになっている.
        if self.force_gameover {
            return true;
        }
        // 初期配置のブロックがすでに配置されたブロックとぶつかっていたらゲームオーバー.
        let block_shape = block_datas::block_shape(*next_block);
        // ブロックの下部には空白があり得るが、初期配置の際にはその分を埋める.
        let padding = block_shape.len() - 1 - block_datas::calc_block_bottom(&block_shape);
        let start_pos = Grid::new(block_datas::BLOCK_START_POSITION.x, block_datas::BLOCK_START_POSITION.y + padding as i32);
        self.check_collision(&block_shape, &start_pos)
    }
    
    /// ブロックがフィールドと衝突するかどうかを返す.
    /// positionはblock_shapeの一番左下の座標.
    pub fn check_collision(&self, block_shape: &Vec<Vec<BlockType>>, position: &Grid) -> bool {
        let pos_y_upper = position.y + 1 - block_shape.len() as i32;
        for y in 0..block_shape.len() {
            for x in 0..block_shape[y].len() {
                if block_shape[y][x] != BlockType::None {
                    let grid_x: i32 = position.x + x as i32;
                    let grid_y: i32 = pos_y_upper + y as i32;
                    let grid = Grid { x: grid_x, y: grid_y };
                    if !self.check_position_in_field(&grid) {
                        return true;
                    }
                    else if self.grid_data[grid_y as usize][grid_x as usize] != BlockType::None {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    /// ブロックの配置予測を出す.
    /// 返ってくる位置はblock_shapeの一番左下の座標.
    pub fn get_ghost_position(&self, block_shape: &Vec<Vec<BlockType>>, now_position: &Grid) -> Grid {
        if block_shape.iter().map(|line| line.iter().all(|cell| *cell == BlockType::None)).all(|b| b) {
            // 全部Noneだったら判定出来ないので、とりあえずもらった値をそのまま返す.
            return now_position.clone();
        }
        let mut ghost_position = now_position.clone();
        let mut next_ghost_position = Grid {x: now_position.x, y: now_position.y + 1};
        while !self.check_collision(block_shape, &next_ghost_position) {
            ghost_position = next_ghost_position;
            next_ghost_position = Grid { x:ghost_position.x, y: ghost_position.y + 1};
        }
        ghost_position
    }
    
    /// フィールドにブロックを固定する.
    /// positionはblock_shapeの一番左下の座標.
    pub fn lock_block(&mut self, block_shape: &Vec<Vec<BlockType>>, position: &Grid) {
        let pos_y_upper = position.y + 1 - block_shape.len() as i32;
        let mut put_in_field = false;
        for y in 0..block_shape.len() {
            for x in 0..block_shape[y].len() {
                if block_shape[y][x] != BlockType::None {
                    let grid_x: i32 = position.x + x as i32;
                    let grid_y: i32 = pos_y_upper + y as i32;
                    if self.check_position_in_field(&Grid{ x: grid_x, y: grid_y }) {
                        self.grid_data[grid_y as usize][grid_x as usize] = block_shape[y][x];
                        put_in_field = true;
                    }
                }
            }
        }
        if !put_in_field {
            // ブロックが完全にフィールド外に配置された場合はゲームオーバー.
            self.force_gameover = true;
        }
    }
    
    /// 攻撃を受け入れて下部にラインを増やす.
    pub fn apply_attack(&mut self, up_lines: usize) {
        // 押し上げて…
        for y in 0..FIELD_HEIGHT_WITH_OUTSIDE {
            for x in 0..FIELD_WIDTH {
                if y < up_lines {
                    if self.grid_data[y][x] != BlockType::None {
                        // 押し上げで枠を越えたらゲームオーバー.
                        self.force_gameover = true;
                    }
                    continue;
                }
                let to_y = y - up_lines;
                self.grid_data[to_y][x] = self.grid_data[y][x];
            }
        }
        // お邪魔を配置.
        let open_pos_x = rand::rng().random::<u32>() as usize % FIELD_WIDTH;
        let put_start_y = FIELD_HEIGHT_WITH_OUTSIDE - up_lines;
        for y in put_start_y..FIELD_HEIGHT_WITH_OUTSIDE {
            for x in 0..FIELD_WIDTH {
                if x == open_pos_x {
                    self.grid_data[y][x] = BlockType::None;
                }
                else{
                    self.grid_data[y][x] = BlockType::Attacked;
                }
            }
        }
    }

    /// 指定した位置の状態を取得する.
    pub fn get_grid_data(&self, position: &Grid) -> BlockType {
        if self.check_position_in_field(position) {
            self.grid_data[position.y as usize][position.x as usize]
        } else {
            BlockType::None
        }
    }
    
    /// フィールド全体の状態を取得する.
    pub fn get_all_grid_data(&self) -> Vec<Vec<BlockType>> {
        vector_util::array_to_vec_2d(self.grid_data)
    }
    
    /// positionがフィールド内にあるかどうかを返す.
    pub fn check_position_in_field(&self, position: &Grid) -> bool {
        position.x >= 0 && position.x < FIELD_WIDTH as i32 &&
        position.y >= 0 && position.y < FIELD_HEIGHT_WITH_OUTSIDE as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_creation() {
        let field = Field::new();
        for y in 0..FIELD_HEIGHT_WITH_OUTSIDE {
            for x in 0..FIELD_WIDTH {
                assert_eq!(field.get_grid_data(&Grid { x: x as i32, y: y as i32 }), BlockType::None);
            }
        }
    }

    #[test]
    fn test_lock_and_get_grid_data() {
        let mut field = Field::new();
        let block_shape = block_datas::block_shape(BlockType::I);
        let position = Grid { x: 3, y: 2 };
        field.lock_block(&block_shape, &position);
        assert_eq!(field.get_grid_data(&Grid { x: 3, y: 1 }), BlockType::I);
        assert_eq!(field.get_grid_data(&Grid { x: 4, y: 1 }), BlockType::I);
        assert_eq!(field.get_grid_data(&Grid { x: 5, y: 1 }), BlockType::I);
    }

    #[test]
    fn test_clear_lines() {
        let mut field = Field::new();
        let last_y = FIELD_HEIGHT_WITH_OUTSIDE - 1;
        for x in 0..FIELD_WIDTH {
            field.grid_data[last_y][x] = BlockType::I;
        }
        let cleared_lines = field.clear_lines();
        assert_eq!(cleared_lines, 1);
        for x in 0..FIELD_WIDTH {
            assert_eq!(field.get_grid_data(&Grid { x: x as i32, y: last_y as i32 }), BlockType::None);
        }
    }

    #[test]
    fn test_drop_lines() {
        let mut field = Field::new();
        let last_y = FIELD_HEIGHT_WITH_OUTSIDE - 1;
        for x in 0..FIELD_WIDTH {
            field.grid_data[last_y - 1][x] = BlockType::I;
        }
        field.drop_lines();
        for x in 0..FIELD_WIDTH {
            assert_eq!(field.get_grid_data(&Grid { x: x as i32, y: last_y as i32}), BlockType::I);
            assert_eq!(field.get_grid_data(&Grid { x: x as i32, y: last_y as i32 - 1 }), BlockType::None);
        }
    }
}