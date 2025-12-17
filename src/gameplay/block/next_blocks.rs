//! 次のブロックを制御する.
//! 全種類ランダムに1つずつ出してから、次は別の順番でまた各種1つずつ出す、という風にできる.
use crate::gameplay::block::block_datas::BlockType;
use rand::seq::SliceRandom;

/// 次以降に出されるブロックを管理する構造体.
/// ブロックの種類数～種類数*2分のブロックの出す順番決められている.
/// ブロックは自動的に補充される.
pub struct NextBlocks {
    bags: [Bag; 2],
    now_bag_index: usize,
}

impl NextBlocks {
    /// 新規インスタンスを作成する.
    pub fn new() -> Self {
        NextBlocks {
            bags: [Bag::new(), Bag::new()],
            now_bag_index: 0,
        }
    }
    /// 次のバッグのインデックスを返す.
    fn next_bag_index(&self) -> usize {
        (self.now_bag_index + 1) % 2
    }

    /// 次のブロックを出す.
    /// ブロックは消費される.
    pub fn next(&mut self) -> BlockType {
        let current_bag = &mut self.bags[self.now_bag_index];
        let next_block = current_bag.next();
        if next_block == BlockType::None {
            self.bags[self.now_bag_index].init();
            self.now_bag_index = self.next_bag_index();
            return self.bags[self.now_bag_index].next();
        }
        next_block
    }

    /// look_ahead個先に出される予定のブロック種類を返す.
    /// まだ決まっていない場合はBlockType::Noneが返る.
    /// ブロックは消費されない.
    pub fn show_next_block(&self, look_ahead: usize) -> BlockType {
        let current_bag = &self.bags[self.now_bag_index];
        let current_rest = current_bag.rest();
        if current_rest <= look_ahead {
            let next_bag = &self.bags[self.next_bag_index()];
            return next_bag.show_next_block(look_ahead - current_rest);
        }
        else{
            return current_bag.show_next_block(look_ahead);
        }
    }
}

const BAG_SIZE: usize = 7;

/// 次に出すブロックのひとかたまり.全種類1個ずつ入っている.
struct Bag {
    blocks: [BlockType; BAG_SIZE],
    index: usize,
}

impl Bag {
    /// 新規インスタンス作成.
    /// ランダム化が含まれているので、少しだけ重い？
    pub fn new() -> Self {
        let mut bag = Bag {
            blocks: [
                BlockType::I,
                BlockType::O,
                BlockType::T,
                BlockType::S,
                BlockType::Z,
                BlockType::J,
                BlockType::L,
            ],
            index: 0,
        };
        bag.init();
        bag
    }

    /// 中身を入れ直してランダム化する.
    pub fn init(&mut self) {
        let mut rng = rand::rng();
        self.blocks.shuffle(&mut rng);
        self.index = 0;
    }

    /// 次の中身を取り出す.
    /// 中身は消費される.
    pub fn next(&mut self) -> BlockType {
        if self.index >= BAG_SIZE {
            return BlockType::None;
        }
        let block = self.blocks[self.index];
        self.index += 1;
        block
    }

    /// look_ahead個先に取り出されるブロックの種類を返す.
    /// 中身の量よりも大きい値が指定された場合はBlockType::Noneが返る.
    /// 中身は消費されない.
    pub fn show_next_block(&self, look_ahead: usize) -> BlockType {
        if self.index + look_ahead >= BAG_SIZE {
            return BlockType::None;
        }
        self.blocks[self.index + look_ahead]
    }
    /// 中身の残量を返す.
    pub fn rest(&self) -> usize {
        BAG_SIZE - self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_blocks() {
        let mut next_blocks = NextBlocks::new();
        let mut results = Vec::new();
        for _ in 0..14 {
            results.push(next_blocks.next());
        }
        assert_eq!(results.len(), 14);
        // 14個のブロックが取得できていることを確認
        for block in results {
            assert_ne!(block, BlockType::None);
        }
    }

    #[test]
    fn test_show_next_block() {
        let next_blocks = NextBlocks::new();
        for i in 0..14 {
            assert_ne!(next_blocks.show_next_block(i), BlockType::None);
        }
    }

    #[test]
    fn test_bag_initialization() {
        let bag = Bag::new();
        assert_eq!(bag.rest(), BAG_SIZE);
    }

    #[test]
    fn test_bag_next() {
        let mut bag = Bag::new();
        let first_block = bag.next();
        assert_ne!(first_block, BlockType::None);
        assert_eq!(bag.rest(), BAG_SIZE - 1);
    }

    #[test]
    fn test_bag_exhaustion() {
        let mut bag = Bag::new();
        for _ in 0..BAG_SIZE {
            assert_ne!(bag.next(), BlockType::None);
        }
        assert_eq!(bag.next(), BlockType::None);
        assert_eq!(bag.rest(), 0);
    }

    #[test]
    fn test_bag_show_next_block() {
        let bag = Bag::new();
        let first_block = bag.show_next_block(0);
        let second_block = bag.show_next_block(1);
        assert_ne!(first_block, BlockType::None);
        assert_ne!(second_block, BlockType::None);
        assert_ne!(first_block, second_block);
    }
}