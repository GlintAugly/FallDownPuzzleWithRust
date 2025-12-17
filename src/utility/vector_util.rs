//! ベクトル操作のための関数をまとめている.

/// 配列をVecに変換する.二次元配列用.
pub fn array_to_vec_2d<T: Clone, const ROWS: usize, const COLS: usize>(array: [[T; COLS]; ROWS]) -> Vec<Vec<T>> {
    array.iter().map(|row| row.to_vec()).collect()
}

/// 右回りに90度回転させる.正方行列でなければ引数のクローンを返す.
pub fn rotate_vec_90_clockwise<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    if rows != cols {
        return matrix.clone();
    }
    let mut rotated = vec![vec![matrix[0][0].clone(); rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            rotated[c][rows - 1 - r] = matrix[r][c];
        }
    }
    rotated
}

/// 左回りに90度回転させる.正方行列でなければ引数のクローンを返す.
pub fn rotate_vec_90_counterclockwise<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    if rows != cols {
        return matrix.clone();
    }
    let mut rotated = vec![vec![matrix[0][0].clone(); rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            rotated[cols - 1 - c][r] = matrix[r][c];
        }
    }
    rotated
}

/// 二次元ベクトルをコピーする.ベクトルの大きさが違ったらコピー出来る分だけコピーしてしまう.
pub fn copy_vec_2d<T: Copy>(target: &mut Vec<Vec<T>>, source: &Vec<Vec<T>>) {
    for y in 0..source.len() {
        if y >= target.len() {
            break;
        }
        for x in 0..source[y].len() {
            if x >= target[y].len() {
                break;
            }
            target[y][x] = source[y][x];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_to_vec_2d() {
        let array = [[1, 2, 3], [4, 5, 6]];
        let vec_2d = array_to_vec_2d(array);
        assert_eq!(vec_2d, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn test_rotate_vec_90_clockwise() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let rotated = rotate_vec_90_clockwise(&matrix);
        assert_eq!(rotated, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_vec_90_counterclockwise() {
        let matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let rotated = rotate_vec_90_counterclockwise(&matrix);
        assert_eq!(rotated, vec![
            vec![3, 6, 9],
            vec![2, 5, 8],
            vec![1, 4, 7],
        ]);
    }
}
