//! スコア計算機.
//! スコアの計算をする.
//! ゲームモードに応じて計算方法を変えてもいいかも.

use crate::gameplay::t_spin_checker::TSpinType;
/// スコア計算機のトレイト
pub trait ScoreCalculator {
    fn calc(&self, eraced_lines: u32, t_spin: TSpinType, combos: u32) -> u64;
}

/// シンプルなスコア計算のための構造体.
pub struct SimpleScoreCalculator {}
impl SimpleScoreCalculator {
    /// 新規インスタンス作成.
    pub fn new() -> Self {
        SimpleScoreCalculator {  }
    }
}
impl ScoreCalculator for SimpleScoreCalculator {
    /// スコア計算.コンボ数は考慮しない.
    fn calc(&self, eraced_lines: u32, t_spin: TSpinType, _: u32) -> u64 {
        // とりあえず1行100点で実装.Tスピンなら2倍
        let multiplier: u64 = match t_spin { 
            TSpinType::Full => 3,
            TSpinType::Mini => 2,
            TSpinType::None => 1
        };
        eraced_lines as u64 * 100 as u64 * multiplier
    }
}

pub trait AttackPowerCalculator {
    fn calc(&self, eraced_lines: u32, t_spin: TSpinType, combo: u32) -> usize;
}

pub struct SimpleAttackPowerCalculator {}
impl SimpleAttackPowerCalculator {
    pub fn new() -> Self {
        SimpleAttackPowerCalculator {  }
    }
}
impl AttackPowerCalculator for SimpleAttackPowerCalculator {
    /// 攻撃ライン数計算.消したラインをそのまま送ることにする.
    fn calc(&self, eraced_lines:u32, _: TSpinType, _:u32) -> usize {
        eraced_lines as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_score() {
        let calculator = SimpleScoreCalculator::new();
        assert_eq!(calculator.calc(4, TSpinType::Full, 0), 1200);
    }

    #[test]
    fn test_calc_attack_power() {
        let calculator = SimpleAttackPowerCalculator::new();
        assert_eq!(calculator.calc(2, TSpinType::Full, 2), 2);
    }
}