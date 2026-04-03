//! https://leetcode.com/problems/power-of-four/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) != 0
    }

    fn alt_solution(n: i32) -> bool {
        n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_power_of_four() {
        assert_eq!(true, Solution::is_power_of_four(16));
        assert_eq!(true, Solution::is_power_of_four(1));
        assert_eq!(false, Solution::is_power_of_four(8));
        assert_eq!(false, Solution::is_power_of_four(5));
        assert_eq!(false, Solution::is_power_of_four(-16));
    }

    #[test]
    fn test_alt_solution() {
        assert_eq!(true, Solution::alt_solution(16));
        assert_eq!(true, Solution::alt_solution(1));
        assert_eq!(false, Solution::alt_solution(8));
        assert_eq!(false, Solution::alt_solution(5));
        assert_eq!(false, Solution::alt_solution(-16));
    }
}
