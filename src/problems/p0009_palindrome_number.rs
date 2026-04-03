//! # 9. Palindrome Number
//!
//! Given an integer `x`, return true if `x` is a palindrome, and false otherwise.
//!
//! ### Example 1:
//! ```text
//! Input: x = 121
//! Output: true
//! Explanation: "121" reads as "121" from left to right and from right to left.
//! ```
//!
//! ### Example 2:
//! ```text
//! Input: x = -121
//! Output: false
//! Explanation: From left to right, it reads "-121".
//! From right to left, it becomes "121-". Therefore it is not a palindrome.
//! ```
//!
//! ### Example 3:
//!
//! ```text
//! Input: x = 10
//! Output: false
//! Explanation: Reads "01" from right to left. Therefore it is not a palindrome.
//! ```
//!
//! ## Constraints:
//!
//! ```text
//! -231 <= x <= 231 - 1
//! ```
//!
//! ## Follow up
//!
//! Could you solve it without converting the integer to a string?
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut reversed = 0;
        let mut num = x;

        while num > 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }

        x == reversed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
    }
}
