//! https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut current_depth = 0;
        let mut max_depth = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    current_depth += 1;
                    max_depth = max_depth.max(current_depth);
                }
                ')' => current_depth -= 1,
                _ => (),
            }
        }

        max_depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(Solution::max_depth("(()(()))".to_string()), 3);
        assert_eq!(Solution::max_depth(")()())".to_string()), 0);
    }
}
