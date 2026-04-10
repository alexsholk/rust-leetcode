//! https://leetcode.com/problems/minimum-distance-between-three-equal-elements-i/
use std::collections::HashMap;

pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut ans = i32::MAX;

        for (i, &val) in nums.iter().enumerate() {
            let entry = map.entry(val).or_insert((-1, -1));
            let (first, second) = entry;

            if *first != -1 {
                let dist = 2 * (i as i32 - *first);
                ans = ans.min(dist);
            }

            *first = *second;
            *second = i as i32;
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::minimum_distance(vec![1, 2, 1, 1, 3]), 6);
        assert_eq!(Solution::minimum_distance(vec![1, 1, 2, 3, 2, 1, 2]), 8);
        assert_eq!(Solution::minimum_distance(vec![1]), -1);
    }
}
