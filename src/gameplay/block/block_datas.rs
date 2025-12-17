//! ブロックのデータ定義と関連した関数.

use crate::utility::grid::Grid;
use crate::utility::vector_util;

/// ブロックを操作可能にする場合に、配置される場所.
pub const BLOCK_START_POSITION_Y: i32 = 17;
pub const BLOCK_START_POSITION: Grid = Grid { x: 2, y: BLOCK_START_POSITION_Y };

/// ブロックの種類.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockType {
    I, L, J, T, Attacked, None
}

const BLOCK_I: [[BlockType; 3]; 3] = [
    [BlockType::None, BlockType::None, BlockType::None],
    [BlockType::I, BlockType::I, BlockType::I],
    [BlockType::None, BlockType::None, BlockType::None],
];
const BLOCK_J: [[BlockType; 3]; 3] = [
    [BlockType::None, BlockType::J, BlockType::None],
    [BlockType::None, BlockType::J, BlockType::J],
    [BlockType::None, BlockType::None, BlockType::None],
];
const BLOCK_L: [[BlockType; 3]; 3] = [
    [BlockType::None, BlockType::L, BlockType::None],
    [BlockType::L, BlockType::L, BlockType::None],
    [BlockType::None, BlockType::None, BlockType::None],
];
const BLOCK_T: [[BlockType; 3]; 3] = [
    [BlockType::None, BlockType::T, BlockType::None],
    [BlockType::T, BlockType::T, BlockType::T],
    [BlockType::None, BlockType::None, BlockType::None],
];

/// 種類に応じたブロックの初期形.
pub fn block_shape(block_type: BlockType) -> Vec<Vec<BlockType>> {
    match block_type {
        BlockType::I => vector_util::array_to_vec_2d(BLOCK_I),
        BlockType::T => vector_util::array_to_vec_2d(BLOCK_T),
        BlockType::J => vector_util::array_to_vec_2d(BLOCK_J),
        BlockType::L => vector_util::array_to_vec_2d(BLOCK_L),
        BlockType::Attacked => vec!(vec!(BlockType::Attacked)),
        BlockType::None => vec!(vec!(BlockType::None)),
    }
}

// ブロックの下のどこまでにブロックが存在しているかを調べて返す.
pub fn calc_block_bottom(block: &Vec<Vec<BlockType>>) -> usize {
    for i in (0..block.len()).rev() {
        for cell in block[i].iter() {
            if *cell != BlockType::None {
                return i;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_shape() {
        let t_shape = block_shape(BlockType::T);
        assert_eq!(t_shape.len(), 3);
        assert_eq!(t_shape[0].len(), 3);
        assert_eq!(t_shape[1][0], BlockType::T);
        assert_eq!(t_shape[1][1], BlockType::T);
        assert_eq!(t_shape[1][2], BlockType::T);
    }
    #[test]
    fn test_calc_block_bottom() {
        assert_eq!(calc_block_bottom(&block_shape(BlockType::I)), 1);
    }
}