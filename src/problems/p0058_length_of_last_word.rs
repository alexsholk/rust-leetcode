//! https://leetcode.com/problems/length-of-last-word/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut end = s.len();
        let bytes = s.as_bytes();
        while end > 0 && bytes[end - 1] == b' ' {
            end -= 1;
        }

        let mut start = end;
        while start > 0 && bytes[start -1] != b' ' {
            start -= 1;
        }

        (end - start) as i32
    }

    pub fn alt_solution(s: String) -> i32 {
        s.trim().split_whitespace().last().map(|s| s.len() as i32).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(Solution::length_of_last_word("a".to_string()), 1);
    }

    #[test]
    fn test_alt_solution() {
        assert_eq!(Solution::alt_solution("Hello World".to_string()), 5);
        assert_eq!(Solution::alt_solution("   fly me   to   the moon  ".to_string()), 4);
        assert_eq!(Solution::alt_solution("a".to_string()), 1);
    }
}