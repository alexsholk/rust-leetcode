//! https://leetcode.com/problems/3sum/
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicates
            }

            if nums[i] > 0 {
                break;
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                match nums[i] + nums[l] + nums[r] {
                    0 => {
                        // found a triplet with sum 0
                        result.push(vec![nums[i], nums[l], nums[r]]);

                        while l < r && nums[l] == nums[l + 1] {
                            l += 1; // skip duplicates
                        }

                        while l < r && nums[r] == nums[r - 1] {
                            r -= 1; // skip duplicates
                        }

                        l += 1;
                        r -= 1;
                    }
                    sum if sum < 0 => l += 1, // increase left pointer to increase sum
                    _ => r -= 1,              // > 0; decrease right pointer to decrease sum
                }
            }
        }

        result
    }

    // Slow, O(n^3)
    fn alt_solution(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        let mut res = Vec::new();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }

            for j in i + 1..nums.len() {
                if nums[i] + nums[j] > 0 {
                    break;
                }

                for k in j + 1..nums.len() {
                    match nums[i] + nums[j] + nums[k] {
                        n if n > 0 => break,
                        0 => {
                            let t = vec![nums[i], nums[j], nums[k]];
                            if !res.contains(&t) {
                                res.push(t);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(
            Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]),
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2]
            ]
        );
    }

    #[test]
    fn test_alt_solution() {
        assert_eq!(
            Solution::alt_solution(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::alt_solution(vec![0, 1, 1]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(Solution::alt_solution(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
        assert_eq!(
            Solution::alt_solution(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]),
            vec![
                vec![-4, -2, 6],
                vec![-4, 0, 4],
                vec![-4, 1, 3],
                vec![-4, 2, 2],
                vec![-2, -2, 4],
                vec![-2, 0, 2]
            ]
        );
    }
}
