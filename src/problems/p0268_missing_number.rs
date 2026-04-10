//! https://leetcode.com/problems/missing-number/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        let expected_sum = n * (n + 1) / 2;
        let actual_sum: i32 = nums.iter().sum();

        expected_sum - actual_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        let nums = vec![3, 0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
        let nums = vec![0, 1];
        assert_eq!(Solution::missing_number(nums), 2);
    }
}
