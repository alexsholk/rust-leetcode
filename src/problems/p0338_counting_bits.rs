//! https://leetcode.com/problems/counting-bits/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; (n + 1) as usize];

        for i in 1..=n as usize {
            res[i] = res[i >> 1] + (i & 1) as i32;
        }

        res
    }

    fn alt_solution(n: i32) -> Vec<i32> {
        let mut c = 0;
        (0..=n)
            .map(|mut i| {
                c = 0;
                while i > 0 {
                    if i & 1 == 1 {
                        c += 1;
                    }
                    i >>= 1;
                }
                c
            })
            .collect()
    }

    fn alt_solution2(n: i32) -> Vec<i32> {
        (0..=n).map(|i| i.count_ones() as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::count_bits(5));
    }

    #[test]
    fn test_alt_solution() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::alt_solution(5));
    }

    #[test]
    fn test_alt_solution2() {
        assert_eq!(vec![0, 1, 1, 2, 1, 2], Solution::alt_solution2(5));
    }
}
