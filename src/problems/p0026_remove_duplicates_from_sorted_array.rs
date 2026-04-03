//! https://leetcode.com/problems/remove-duplicates-from-sorted-array/
pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut k = 1;

        for i in 1..nums.len() {
            if nums[i] != nums[k - 1] {
                nums[k] = nums[i];
                k += 1;
            }
        }

        nums.truncate(k);

        k as i32
    }

    fn alt_solution(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6];
        assert_eq!(6, Solution::remove_duplicates(&mut nums));
        assert_eq!(vec![1, 2, 3, 4, 5, 6], nums);
    }
}
