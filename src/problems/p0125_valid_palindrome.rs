//! https://leetcode.com/problems/valid-palindrome/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = bytes.len().saturating_sub(1);

        while left < right {
            let l = bytes[left] as char;
            let r = bytes[right] as char;

            if !l.is_alphanumeric() {
                left += 1;
                continue;
            }

            if !r.is_alphanumeric() {
                right -= 1;
                continue;
            }

            if l.to_ascii_lowercase() != r.to_ascii_lowercase() {
                return false;
            }

            left += 1;
            right = right.saturating_sub(1);
        }

        true
    }

    fn alt_solution(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();

        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }

    #[test]
    fn test_alt_solution() {
        assert_eq!(Solution::alt_solution("A man, a plan, a canal: Panama".to_string()), true);
        assert_eq!(Solution::alt_solution("race a car".to_string()), false);
        assert_eq!(Solution::alt_solution(" ".to_string()), true);
    }
}
