//! https://leetcode.com/problems/move-zeroes/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut pos = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[pos] = nums[i];
                pos += 1;
            }
        }

        for i in pos..nums.len() {
            nums[i] = 0;
        }
    }

    pub fn alt_solution(nums: &mut Vec<i32>) {
        let mut queue = std::collections::VecDeque::new();

        for i in 0..nums.len() {
            if nums[i] == 0 {
                queue.push_back(i);
            } else if let Some(n) = queue.pop_front() {
                nums[n] = nums[i];
                nums[i] = 0;
                queue.push_back(i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut input = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut input);
        assert_eq!(input, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn test_alt_solution() {
        let mut input = vec![0, 1, 0, 3, 12];
        Solution::alt_solution(&mut input);
        assert_eq!(input, vec![1, 3, 12, 0, 0]);
    }
}
