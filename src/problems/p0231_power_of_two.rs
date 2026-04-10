//! https://leetcode.com/problems/power-of-two/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && n.count_ones() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert_eq!(Solution::is_power_of_two(1), true);
        assert_eq!(Solution::is_power_of_two(16), true);
        assert_eq!(Solution::is_power_of_two(3), false);
    }
}
