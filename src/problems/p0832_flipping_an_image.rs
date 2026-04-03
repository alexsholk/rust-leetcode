//! https://leetcode.com/problems/flipping-an-image/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for row in &mut image {
            row.reverse();
            for x in row {
                *x ^= 1;
            }
        }
        image
    }

    fn alt_solution(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .into_iter()
            .map(|row| row.into_iter().rev().map(|x| 1 - x).collect())
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_and_invert_image() {
        let input = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let expected_output = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let output = Solution::flip_and_invert_image(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_alt_solution() {
        let input = vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]];
        let expected_output = vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let output = Solution::alt_solution(input);
        assert_eq!(output, expected_output);
    }
}
