//! https://leetcode.com/problems/valid-sudoku/
use std::ops::Range;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        (0..9).all(|i| {
            [BoardArea::row(i), BoardArea::column(i), BoardArea::block(i)]
                .iter()
                .all(|area| area.is_valid(&board))
        })
    }
}

struct BoardArea {
    rows: Range<usize>,
    cols: Range<usize>,
}

impl BoardArea {
    fn new(rows: Range<usize>, cols: Range<usize>) -> Self {
        Self { rows, cols }
    }

    fn row(n: usize) -> Self {
        Self::new(n..n + 1, 0..9)
    }

    fn column(n: usize) -> Self {
        Self::new(0..9, n..n + 1)
    }

    fn block(n: usize) -> Self {
        Self::new(
            (n / 3) * 3..((n / 3) + 1) * 3,
            (n % 3) * 3..((n % 3) + 1) * 3,
        )
    }

    fn is_valid(&self, board: &[Vec<char>]) -> bool {
        let mut bits = 0u16;
        for i in self.rows.clone() {
            for j in self.cols.clone() {
                let c = board[i][j];

                if c == '.' {
                    continue;
                }

                let digit = (c as u8 - b'1') as u16;
                let bit = 1 << digit;

                if bits & bit != 0 {
                    return false;
                }

                bits |= bit;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(board), true);
    }
}
