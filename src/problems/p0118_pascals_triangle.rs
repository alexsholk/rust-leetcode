//! https://leetcode.com/problems/pascals-triangle/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);

        for i in 0..num_rows as usize {
            let mut row = Vec::with_capacity(i + 1);

            for j in 0..=i {
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    let prev = &result[i - 1];
                    row.push(prev[j - 1] + prev[j]);
                }
            }

            result.push(row);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
        assert_eq!(Solution::generate(1), vec![vec![1]])
    }
}
