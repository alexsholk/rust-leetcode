//! https://leetcode.com/problems/pascals-triangle-ii/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        let mut row = vec![1; n + 1];

        for i in 2..=n {
            for j in (1..i).rev() {
                row[j] += row[j - 1];
            }
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        let row_index = 3;
        let expected = vec![1, 3, 3, 1];
        let result = Solution::get_row(row_index);
        assert_eq!(result, expected);

        let row_index = 0;
        let expected = vec![1];
        let result = Solution::get_row(row_index);
        assert_eq!(result, expected);

        let row_index = 1;
        let expected = vec![1, 1];
        let result = Solution::get_row(row_index);
        assert_eq!(result, expected);
    }
}
