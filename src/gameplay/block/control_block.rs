//! 操作できるブロックのための処理.

use crate::utility::grid::Grid;
use crate::gameplay::block::block_datas::{self, BlockType};
use crate::utility::vector_util;
use crate::gameplay::field::Field;

/// 操作されるブロックの構造体.
#[derive(PartialEq, Clone)]
pub struct ControlBlock {
    // 現在操作中のブロックの位置.一番左下の座標とする.
    pub position: Grid,
    pub block: Vec<Vec<BlockType>>,
    pub block_type: BlockType,
}

impl ControlBlock {
    /// 新規インスタンス作成.
    pub fn new() -> Self {
        ControlBlock {
            position: Grid::new(0, 0),
            block: vec![],
            block_type: BlockType::None,
        }
    
    }
    
    /// ブロックの割り当て.
    pub fn apply_block(&mut self, block_type: BlockType, start_position: Grid) {
        self.block_type = block_type;
        self.block = block_datas::block_shape(block_type);
        self.position = start_position;
        // ブロック下部の空白は先に埋めてしまう.
        let padding = self.block.len() - 1 - block_datas::calc_block_bottom(&self.block);
        self.position.y += padding as i32;
    }

    /// 割り当てブロックの削除.
    pub fn delete_block(&mut self) {
        self.position = Grid::new(0, 0);
        self.block = vec![];
        self.block_type =  BlockType::None;
    }

    /// 下移動.移動出来ない場合は何もしない.
    pub fn down(&mut self, field: &Field)  -> bool {
        if self.block_type != BlockType::None && !field.check_collision(&self.block, &Grid::new(self.position.x, self.position.y + 1)) {
            self.position.y += 1;
            return true;
        }
        false
    }

    /// 最下部まで落とす.
    pub fn hard_drop(&mut self, field: &Field) -> bool {
        while self.block_type != BlockType::None && !field.check_collision(&self.block, &Grid::new(self.position.x, self.position.y + 1)) {
            self.position.y += 1;
        }
        true
    }

    /// 左移動.移動出来ない場合は何もしない.
    pub fn left(&mut self, field: &Field) -> bool {
        if self.block_type != BlockType::None && !field.check_collision(&self.block, &Grid::new(self.position.x -1 , self.position.y)) {
            self.position.x -= 1;
            return true;
        }
        false
    }

    /// 右移動.移動出来ない場合は何もしない.
    pub fn right(&mut self, field: &Field) -> bool {
        if self.block_type != BlockType::None && !field.check_collision(&self.block, &Grid::new(self.position.x + 1, self.position.y)) {
            self.position.x += 1;
            return true;
        }
        false
    }

    /// 右回転.回転出来ない場合は何もしない.
    /// 単に回転するのではなく、障害物に応じて多少移動する.
    pub fn rotate(&mut self, field: &Field) -> bool {
        if self.block_type == BlockType::None {
            return false;
        }
        let rotated = vector_util::rotate_vec_90_clockwise(&self.block);
        let compensation = match self.block_type {
            BlockType::I =>  [Grid::new(0, 0), Grid::new(-2, 0), Grid::new(1, 0), Grid::new(-2, 1), Grid::new(-1, -2)],
            _ => [Grid::new(0, 0), Grid::new(-1, 0), Grid::new(-1, -1), Grid::new(0, 2), Grid::new(-1, 2)],
        };
        for offset in compensation.iter() {
            let test_position = Grid::new(self.position.x + offset.x, self.position.y + offset.y);
            if !field.check_collision(&rotated, &test_position) {
                self.position = test_position;
                self.block = rotated;
                return true;
            }
        }
        false
    }

    /// 左回転.回転出来ない場合は何もしない.
    /// 単に回転するのではなく、障害物に応じて多少移動する.
    pub fn counter_rotate(&mut self, field: &Field) -> bool {
        if self.block_type == BlockType::None {
            return false;
        }
        let rotated = vector_util::rotate_vec_90_counterclockwise(&self.block);
        let compensation = match self.block_type {
            BlockType::I =>  [Grid::new(0, 0), Grid::new(-1, 0), Grid::new(2, 0), Grid::new(-1, -2), Grid::new(2, 1)],
            _ => [Grid::new(0, 0), Grid::new(1, 0), Grid::new(1, -1), Grid::new(0, 2), Grid::new(1, 2)],
        };
        for offset in compensation.iter() {
            let test_position = Grid::new(self.position.x + offset.x, self.position.y + offset.y);
            if !field.check_collision(&rotated, &test_position) {
                self.position = test_position;
                self.block = rotated;
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameplay::field;

    #[test]
    fn test_rotate() {
        let mut control_block = ControlBlock::new();
        control_block.apply_block(BlockType::T, block_datas::BLOCK_START_POSITION);
        let field = Field::new();
        control_block.rotate(&field);
        assert_eq!(control_block.block, vec![
            vec![BlockType::None, BlockType::T, BlockType::None],
            vec![BlockType::None, BlockType::T, BlockType::T],
            vec![BlockType::None, BlockType::T, BlockType::None],
        ]);

        control_block.counter_rotate(&field);
        assert_eq!(control_block.block, vec![
            vec![BlockType::None, BlockType::T, BlockType::None],
            vec![BlockType::T, BlockType::T, BlockType::T],
            vec![BlockType::None, BlockType::None, BlockType::None],
        ]);
    }

    #[test]
    fn test_movement() {
        let mut control_block = ControlBlock::new();
        control_block.apply_block(BlockType::I, block_datas::BLOCK_START_POSITION);
        let field = Field::new();
        // ブロックの左下が必ずしもフィールドの一番下に来るわけではないので調整が必要.
        let mut block_position_adjust = 0; 
        'outer: for y in (0..control_block.block.len()).rev() {
        for x in 0..control_block.block[y].len() {
                if control_block.block[y][x] != BlockType::None {
                    block_position_adjust = (control_block.block.len() - 1 - y) as i32;
                    break 'outer;
                }
            }
        };

        control_block.down(&field);
        assert_eq!(control_block.position, Grid::new(2, 18 + block_position_adjust));

        control_block.left(&field);
        assert_eq!(control_block.position, Grid::new(1, 18 + block_position_adjust));

        control_block.right(&field);
        assert_eq!(control_block.position, Grid::new(2, 18 + block_position_adjust));

        control_block.hard_drop(&field);
        assert_eq!(control_block.position.y, field::FIELD_HEIGHT_WITH_OUTSIDE as i32 - 1 + block_position_adjust);
    }
}
