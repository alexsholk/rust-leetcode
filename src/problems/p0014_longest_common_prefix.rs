//! https://leetcode.com/problems/longest-common-prefix/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        strs.sort();

        let first = &strs[0];
        let last = &strs[strs.len() - 1];

        let mut i = 0;
        let bytes1 = first.as_bytes();
        let bytes2 = last.as_bytes();

        while i < bytes1.len() && i < bytes2.len() && bytes1[i] == bytes2[i] {
            i += 1;
        }

        first[..i].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
}
