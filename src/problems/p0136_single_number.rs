//! https://leetcode.com/problems/single-number/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        assert_eq!(Solution::single_number(vec![2,2,1]), 1);
        assert_eq!(Solution::single_number(vec![4,1,2,1,2]), 4);
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}