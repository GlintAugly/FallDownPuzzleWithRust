//! 二次元座標.

use std::ops::{Add, Sub};

/// 二次元座標の構造体.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub x: i32,
    pub y: i32,
}
impl Grid {
    /// 新規インスタンス作成
    pub fn new(x: i32, y: i32) -> Self {
        Grid { x, y }
    }

    /// 構造体データのコピー.
    pub fn copy_data(&mut self, other: &Grid) {
        self.x = other.x;
        self.y = other.y;
    }
}

impl Sub<&Grid> for Grid {
    type Output = Grid;

    /// 減算処理.
    fn sub(mut self, other: &Grid) -> Grid {
        self.x -= other.x;
        self.y -= other.y;
        self
    }
}

impl Add<&Grid> for Grid {
    type Output = Grid;

    /// 加算処理.
    fn add(mut self, other: &Grid) -> Grid{
        self.x += other.x;
        self.y += other.y;
        self
    }
}
