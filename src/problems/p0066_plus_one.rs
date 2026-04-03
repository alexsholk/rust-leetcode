//! https://leetcode.com/problems/plus-one/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;

        for d in digits.iter_mut().rev() {
            let sum = *d + carry;
            carry = sum / 10;
            *d = sum % 10;

            if carry == 0 {
                return digits;
            }
        }

        let mut result = Vec::with_capacity(digits.len() + 1);
        result.push(carry);
        result.extend(digits);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
        assert_eq!(Solution::plus_one(vec![]), vec![1]);
    }
}
