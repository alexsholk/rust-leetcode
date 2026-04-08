//! https://leetcode.com/problems/flip-square-submatrix-vertically/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_submatrix(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let x = x as usize;
        let y = y as usize;
        let k = k as usize;

        for i in 0..k / 2 {
            let row1 = x + i;
            let row2 = x + k - 1 - i;

            for col in y..y + k {
                (grid[row1][col], grid[row2][col]) = (grid[row2][col], grid[row1][col]);
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_submatrix() {
        let grid = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let x = 1;
        let y = 0;
        let k = 3;
        let expected = vec![
            vec![1, 2, 3, 4],
            vec![13, 14, 15, 8],
            vec![9, 10, 11, 12],
            vec![5, 6, 7, 16],
        ];
        assert_eq!(Solution::reverse_submatrix(grid, x, y, k), expected);

        let grid = vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]];
        let x = 0;
        let y = 2;
        let k = 2;
        let expected = vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]];
        assert_eq!(Solution::reverse_submatrix(grid, x, y, k), expected);
    }
}
