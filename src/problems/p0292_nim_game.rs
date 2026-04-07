//! https://leetcode.com/problems/nim-game/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_win_nim() {
        assert_eq!(Solution::can_win_nim(3), true);
        assert_eq!(Solution::can_win_nim(4), false);
        assert_eq!(Solution::can_win_nim(5), true);
        assert_eq!(Solution::can_win_nim(6), true);
        assert_eq!(Solution::can_win_nim(7), true);
        assert_eq!(Solution::can_win_nim(8), false);
    }
}
