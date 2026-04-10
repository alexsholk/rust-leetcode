//! https://leetcode.com/problems/integer-to-roman/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        const ROMAN: [(&str, i32); 13] = [
            ("M", 1000),
            ("CM", 900),
            ("D", 500),
            ("CD", 400),
            ("C", 100),
            ("XC", 90),
            ("L", 50),
            ("XL", 40),
            ("X", 10),
            ("IX", 9),
            ("V", 5),
            ("IV", 4),
            ("I", 1),
        ];

        let mut result = String::with_capacity(16);

        for (symbol, value) in ROMAN {
            result.push_str(&symbol.repeat((num / value) as usize));
            num %= value;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX".to_string());
    }
}
