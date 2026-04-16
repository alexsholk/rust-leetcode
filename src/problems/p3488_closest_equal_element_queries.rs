//! https://leetcode.com/problems/closest-equal-element-queries
use std::collections::HashMap;

pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &x) in nums.iter().enumerate() {
            groups.entry(x).or_default().push(i);
        }

        let mut ans = vec![-1; n];

        for positions in groups.values() {
            let m = positions.len();

            if m == 1 {
                continue;
            }

            for j in 0..m {
                let cur = positions[j];

                let prev = positions[(j + m - 1) % m];
                let next = positions[(j + 1) % m];

                let d1 = Self::dist(cur, prev, n);
                let d2 = Self::dist(cur, next, n);

                ans[cur] = d1.min(d2) as i32;
            }
        }

        queries.into_iter().map(|i| ans[i as usize]).collect()
    }

    fn dist(a: usize, b: usize, n: usize) -> usize {
        let diff = a.abs_diff(b);
        diff.min(n - diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_queries() {
        assert_eq!(
            Solution::solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5]),
            vec![2, -1, 3]
        );

        assert_eq!(
            Solution::solve_queries(vec![1, 2, 3, 4], vec![0, 1, 2, 3]),
            vec![-1, -1, -1, -1]
        );
    }
}
