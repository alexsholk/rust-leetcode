//! https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let w: Vec<i32> = word.chars().map(|c| (c as u8 - b'A') as i32).collect();

        let mut dp = vec![0; 26];
        let mut res = 0;
        let mut best = 0;

        for i in 1..w.len() {
            let b = w[i - 1];
            let c = w[i];
            let d = Self::dist(b, c);

            for a in 0..26 {
                dp[b as usize] = dp[b as usize].max(
                    dp[a] + d - Self::dist(a as i32, c)
                );
            }

            best = best.max(dp[b as usize]);
            res += d;
        }

        res - best
    }

    fn dist(a: i32, b: i32) -> i32 {
        (a / 6 - b / 6).abs() + (a % 6 - b % 6).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_distance() {
        assert_eq!(Solution::minimum_distance("CAKE".to_string()), 3);
        assert_eq!(Solution::minimum_distance("HAPPY".to_string()), 6);
        assert_eq!(Solution::minimum_distance("AAB".to_string()), 0);
        assert_eq!(Solution::minimum_distance("AABCC".to_string()), 1);
    }
}
