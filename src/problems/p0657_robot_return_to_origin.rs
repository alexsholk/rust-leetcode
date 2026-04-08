//! https://leetcode.com/problems/robot-return-to-origin/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        if moves.len() % 2 != 0{
            return false
        }

        let (mut x, mut y) = (0, 0);
        for &b in moves.as_bytes() {
            match b {
                b'U' => y += 1,
                b'D' => y -= 1,
                b'L' => x += 1,
                b'R' => x -= 1,
                _ => unreachable!(),
            }
        }

        (x, y) == (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judge_circle() {
        let moves = "UD".to_string();
        assert_eq!(Solution::judge_circle(moves), true);

        let moves = "LL".to_string();
        assert_eq!(Solution::judge_circle(moves), false);

        let moves = "RRDD".to_string();
        assert_eq!(Solution::judge_circle(moves), false);
    }
}