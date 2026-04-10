//! https://leetcode.com/problems/set-matrix-zeroes/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut first_row_zero = false;
        let mut first_col_zero = false;

        for col in 0..cols {
            if matrix[0][col] == 0 {
                first_row_zero = true;
            }
        }

        for row in 0..rows {
            if matrix[row][0] == 0 {
                first_col_zero = true;
            }
        }

        for row in 1..rows {
            for col in 1..cols {
                if matrix[row][col] == 0 {
                    matrix[row][0] = 0;
                    matrix[0][col] = 0;
                }
            }
        }

        for row in 1..rows {
            if matrix[row][0] == 0 {
                for col in 1..cols {
                    matrix[row][col] = 0;
                }
            }
        }

        for col in 1..cols {
            if matrix[0][col] == 0 {
                for row in 1..rows {
                    matrix[row][col] = 0;
                }
            }
        }

        if first_row_zero {
            for col in 0..cols {
                matrix[0][col] = 0;
            }
        }

        if first_col_zero {
            for row in 0..rows {
                matrix[row][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zeroes() {
        let mut matrix = vec![
            vec![1, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 1],
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1],
        ]);

        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![13, 14, 15, 0],
        ];


        vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 7, 8],
            vec![0, 10, 11, 12],
            vec![0, 14, 15, 0],
        ];

        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![
            vec![0, 0, 3, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ]);
    }
}
