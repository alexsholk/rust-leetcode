//! https://leetcode.com/problems/sqrtx/

pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let mut n = x as i64;

        loop {
            let next = (n + x as i64 / n) / 2;

            if next >= n {
                return n as i32;
            }

            n = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(16), 4);
    }
}
