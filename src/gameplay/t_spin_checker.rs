//! Tスピンに関する処理を行う.

use crate::gameplay::block::{
    control_block::ControlBlock,
    block_datas::{self, BlockType}
};
use crate::utility::grid::Grid;
use crate::utility::vector_util;
use std::collections::HashMap;

/// Tスピンの種別.
/// Tブロック周辺にブロックが充分埋まっていない場合、または直前の動作がスピンでないと判定された場合はNone.
/// Tブロックの突部分の両隣にブロックがある場合、または特殊な移動を含むスピンが行われていた場合はFull
/// それ以外はFalse
#[derive(PartialEq, Copy, Clone)]
pub enum TSpinType {
    None,
    Mini,
    Full,
}
/// Tブロックの方向.
/// Tブロックの凸部分がどの方向を向いているかを返す.
#[derive(PartialEq, Eq, Hash)]
enum TBlockDirection {
    N,
    E,
    W,
    S,
    None,
}

/// Tスピンが行われているかを判定するための構造体.
/// 
pub struct TSpinChecker {
    t_block_data: Vec<Vec<BlockType>>,
    t_block_position: Grid,
}

impl TSpinChecker {
    /// 新しいインスタンスを作る.
    /// ゲーム中使い回すことを想定しているため、個別のパラメータ指定はここではしない.
    pub fn new() -> Self {
        TSpinChecker{
            t_block_data: block_datas::block_shape(BlockType::T),
            t_block_position: Grid::new(0, 0),
        }
    }

    /// Tスピンが行われているかの判定に用いるデータをセットする.
    /// ブロックが移動する直前に呼び出されることを想定している.
    pub fn set_block_data(&mut self, control_block: &ControlBlock){
        if control_block.block_type == BlockType::T {
            // Tスピン判定用データ更新
            vector_util::copy_vec_2d(&mut self.t_block_data, &control_block.block);
            self.t_block_position.copy_data(&control_block.position);
        }
    }

    /// Tスピンが行われているかを判定する.
    /// 事前にset_block_dataを呼び出されていることを想定している.
    pub fn check_t_spinned(&self, control_block: &ControlBlock) -> bool {
        if control_block.block_type == BlockType::T && self.t_block_data != *control_block.block {
            true
        }
        else {
            // Tブロックでないor回転していない移動.
            false
        }
    }

    /// Tスピン判定
    pub fn calc_t_spin_type(&self, control_block: &ControlBlock, field_data: &Vec<Vec<BlockType>>) -> TSpinType {
        // 座標が特定の移動の仕方をしていた場合は、隣が空いていてもフル判定.
        let srs_last_pattern_delta = Grid::new(1, 2);
        let counter_srs_last_pattern_delta = Grid::new(-1, 2);
        let pos_delta = control_block.position.clone() - &self.t_block_position;
        let srs_last_pattern = pos_delta == srs_last_pattern_delta || pos_delta == counter_srs_last_pattern_delta;

        let t_direction = self.search_t_block_direction(&control_block.block);
        if t_direction == TBlockDirection::None {
            // Tブロックじゃなさそう…
            return TSpinType::None;
        }
        // Tスピンのチェック.
        let mut corner_count = 0;
        let mut mini_count = 0;
        let corners = [
            Grid::new(0, 0),
            Grid::new(2, 0),
            Grid::new(0, 2),
            Grid::new(2, 2),
        ];
        // 突部分の隣が空いてたらミニになるので、突部分の隣のインデックスを取っておく.
        let mini_check_corners = match t_direction {
            TBlockDirection::N => [0, 1],
            TBlockDirection::E => [1, 3],
            TBlockDirection::S => [3, 2],
            TBlockDirection::W => [2, 0],
            _ => [4, 4],
        };
        for (i, corner) in corners.iter().enumerate() {
            if field_data[corner.y as usize][corner.x as usize] != BlockType::None {
                corner_count += 1;
                if i == mini_check_corners[0] || i == mini_check_corners[1] {
                    mini_count += 1;
                }
            }
        }
        if corner_count < 3 {
            return TSpinType::None;
        }
        else{
            if mini_count == 2 || srs_last_pattern {
                return TSpinType::Full;
            }
            else{
                return  TSpinType::Mini;
            }
        }
    }

    
    /// Tブロックの方向チェック.
    /// Tブロック以外が入力されることは想定外のため、値の正しさは保障されない.
    fn search_t_block_direction(&self, t_block_data: &Vec<Vec<BlockType>>) -> TBlockDirection {
        // 突部分の逆側が空いていることで、方向を確認する.
        let mut hips = HashMap::new();
        hips.insert(TBlockDirection::N, [1, 2]);
        hips.insert(TBlockDirection::E, [0, 1]);
        hips.insert(TBlockDirection::W, [1, 0]);
        hips.insert(TBlockDirection::S, [2, 1]);
        for (direction, top) in hips {
            if t_block_data[top[1]][top[0]] != BlockType::T {
                return direction;
            }
        }
        TBlockDirection::None
    }
}