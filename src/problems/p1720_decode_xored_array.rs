//! https://leetcode.com/problems/decode-xored-array/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut result = vec![first; encoded.len() + 1];

        for (i, &v) in encoded.iter().enumerate() {
            result[i + 1] = result[i] ^ v;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(Solution::decode(vec![1,2,3], 1), vec![1,0,2,1]);
        assert_eq!(Solution::decode(vec![6,2,7,3], 4), vec![4,2,0,7,4]);
    }
}