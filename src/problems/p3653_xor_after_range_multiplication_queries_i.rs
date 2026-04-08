//! https://leetcode.com/problems/xor-after-range-multiplication-queries-i/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: u64 = 1_000_000_007;

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as u64;

            let mut i = l;

            while i <= r {
                let x = nums[i] as u64 * v;
                let a = x % MOD;
                nums[i] = a as i32;
                i += k;
            }
        }

        nums.iter().fold(0, |acc, &x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_after_queries() {
        let nums = vec![1, 1, 1];
        let queries = vec![vec![0, 2, 1, 4]];
        let expected = 4;
        assert_eq!(Solution::xor_after_queries(nums, queries), expected);
    }
}
