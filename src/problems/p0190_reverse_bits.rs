//! https://leetcode.com/problems/reverse-bits/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut reversed: u32 = 0;

        for i in 0..32 {
            reversed <<= 1;
            reversed |= (n as u32 >> i) & 1;
        }

        reversed as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(Solution::reverse_bits(0b00000010100101000001111010011100), 0b00111001011110000010100101000000);
        assert_eq!(Solution::reverse_bits(0b01111111111111111111111111111111_u32 as i32), 0b11111111111111111111111111111110_u32 as i32);
    }
}