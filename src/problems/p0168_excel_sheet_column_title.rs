//! https://leetcode.com/problems/excel-sheet-column-title/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut title = String::new();

        while column_number > 0 {
            column_number -= 1;
            let remainder = (column_number % 26) as u8;
            title.push((b'A' + remainder) as char);
            column_number /= 26;
        }

        title.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(Solution::convert_to_title(1), "A");
        assert_eq!(Solution::convert_to_title(2), "B");
        assert_eq!(Solution::convert_to_title(25), "Y");
        assert_eq!(Solution::convert_to_title(26), "Z");
        assert_eq!(Solution::convert_to_title(28), "AB");
        assert_eq!(Solution::convert_to_title(701), "ZY");
        assert_eq!(Solution::convert_to_title(702), "ZZ");
        assert_eq!(Solution::convert_to_title(703), "AAA");
    }
}
