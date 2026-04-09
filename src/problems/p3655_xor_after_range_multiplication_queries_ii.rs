//! https://leetcode.com/problems/xor-after-range-multiplication-queries-ii/
// TODO refine solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: u64 = 1_000_000_007;
        let n = nums.len();
        let block = (n as f64).sqrt() as usize + 1;

        let nums = nums.into_iter().map(|x| x as u64).collect::<Vec<_>>();

        let mut mul = vec![1u64; n];

        let mut small = vec![Vec::new(); block + 1];
        let mut large = Vec::new();

        for q in queries {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let k = q[2] as usize;
            let v = q[3] as u64;

            if k <= block {
                small[k].push((l, r, v));
            } else {
                large.push((l, r, k, v));
            }
        }

        for k in 1..=block {
            if small[k].is_empty() {
                continue;
            }

            for r in 0..k {
                let mut idxs = Vec::new();
                let mut i = r;
                while i < n {
                    idxs.push(i);
                    i += k;
                }

                let m = idxs.len();
                let mut diff = vec![1u64; m + 1];

                for &(l, r_bound, v) in &small[k] {
                    if l % k != r {
                        continue;
                    }

                    let start = (l - r) / k;
                    let end = (r_bound - r) / k;

                    diff[start] = diff[start] * v % MOD;
                    if end + 1 < diff.len() {
                        diff[end + 1] = diff[end + 1] * Self::pow_mod(v, MOD - 2, MOD) % MOD;
                    }
                }

                let mut cur = 1u64;
                for j in 0..m {
                    cur = cur * diff[j] % MOD;
                    mul[idxs[j]] = mul[idxs[j]] * cur % MOD;
                }
            }
        }

        for (l, r, k, v) in large {
            let mut i = l;
            while i <= r {
                mul[i] = mul[i] * v % MOD;
                i += k;
            }
        }

        let mut res = 0;
        for i in 0..n {
            let val = nums[i] * mul[i] % MOD;
            res ^= val as i32;
        }

        res
    }

    fn pow_mod(mut x: u64, mut y: u64, m: u64) -> u64 {
        let mut res = 1;
        while y > 0 {
            if y & 1 == 1 {
                res = res * x % m;
            }
            x = x * x % m;
            y >>= 1;
        }
        res
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
