//! https://leetcode.com/problems/number-of-1-bits/description/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        // Best way but too simple
        // n.count_ones() as i32

        let mut count = 0;
        for i in 0..32 {
            if n & (1 << i) != 0 {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        assert_eq!(
            Solution::hamming_weight(0b00000000000000000000000000001011),
            3
        );
        assert_eq!(
            Solution::hamming_weight(0b00000000000000000000000010000000),
            1
        );
        assert_eq!(
            Solution::hamming_weight(0b01111111111111111111111111111111),
            31
        );
    }
}
