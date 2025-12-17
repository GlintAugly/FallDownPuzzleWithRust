//! ブロックをホールドする機能.
//! 今操作しているブロックとホールドしていたブロックを入れ替えることができる.
//! 
use crate::gameplay::block::block_datas::BlockType;

/// 現在ホールドしているブロックを持つ構造体.
pub struct HoldBlock {
    holding_block: BlockType,
    can_hold: bool,
}

impl HoldBlock {
    /// 新規インスタンス作成.
    pub fn new() -> Self {
        HoldBlock {
            holding_block: BlockType::None,
            can_hold: true,
        }
    }
    
    /// ブロックをホールドして、それまでホールドしていたブロックを返す.
    /// 一度ホールドすると、[allow_hold]を呼び出すまでホールド出来ない.
    /// ホールド出来なかった場合、Noneを返す.
    pub fn hold(&mut self, current_block: BlockType) -> Option<BlockType> {
        if !self.can_hold {
            return None;
        }
        self.can_hold = false;
        let temp = self.holding_block;
        self.holding_block = current_block;
        Some(temp)
    }

    pub fn get_holding_block(&self) -> BlockType {
        self.holding_block
    }
    /// ホールド出来るかどうかを返す.
    pub fn can_hold(&self) -> bool {
        self.can_hold
    }

    /// ホールド出来るようにする.
    pub fn allow_hold(&mut self) {
        self.can_hold = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hold_block() {
        let mut hold_block = HoldBlock::new();
        assert_eq!(hold_block.holding_block, BlockType::None);
        assert_eq!(hold_block.can_hold, true);

        let returned_block = hold_block.hold(BlockType::T).unwrap();
        assert_eq!(returned_block, BlockType::None);
        assert_eq!(hold_block.holding_block, BlockType::T);
        assert_eq!(hold_block.can_hold, false);
        
        let returned_block = hold_block.hold(BlockType::I);
        assert_eq!(returned_block, None);
        assert_eq!(hold_block.holding_block, BlockType::T);

        hold_block.allow_hold();
        let returned_block = hold_block.hold(BlockType::I).unwrap();
        assert_eq!(returned_block, BlockType::T);
        assert_eq!(hold_block.holding_block, BlockType::I);
    }
}
