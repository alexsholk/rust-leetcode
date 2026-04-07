//! https://leetcode.com/problems/reverse-integer/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut result = 0;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            if result > i32::MAX / 10 || result < i32::MIN / 10 {
                return 0;
            }

            result = result * 10 + digit;
        }

        result
    }

    pub fn alt_solution(x: i32) -> i32 {
        let mut str = x.abs().to_string();
        if x < 0 {
            str.push('-');
        }
        str.chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(-2147483648), 0);
    }

    #[test]
    fn test_alt_solution() {
        assert_eq!(Solution::alt_solution(123), 321);
        assert_eq!(Solution::alt_solution(-123), -321);
        assert_eq!(Solution::alt_solution(120), 21);
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
