//! https://leetcode.com/problems/walking-robot-simulation-ii/
//! TODO refactor

use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Vector2d {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::East => write!(f, "East"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
        }
    }
}

#[allow(dead_code)]
struct Robot {
    width: i32,
    height: i32,
    perimeter: i32,
    offset: i32,
}

#[allow(dead_code)]
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        let perimeter = 2 * (width + height) - 4;

        Self {
            width,
            height,
            perimeter,
            offset: 0,
        }
    }

    fn step(&mut self, num: i32) {
        if self.perimeter == 0 {
            return;
        }

        let mut next = (self.offset + num) % self.perimeter;

        if next == 0 && num > 0 {
            next = self.perimeter;
        }

        self.offset = next;
    }

    fn get_pos(&self) -> Vec<i32> {
        let (pos, _) = self.decode();
        vec![pos.x, pos.y]
    }

    fn get_dir(&self) -> String {
        let (_, dir) = self.decode();
        dir.to_string()
    }

    fn decode(&self) -> (Vector2d, Direction) {
        let w = self.width;
        let h = self.height;

        let mut k = self.offset;

        if k <= w - 1 {
            return (Vector2d { x: k, y: 0 }, Direction::East);
        }
        k -= w - 1;

        if k <= h - 1 {
            return (Vector2d { x: w - 1, y: k }, Direction::North);
        }
        k -= h - 1;

        if k <= w - 1 {
            return (
                Vector2d {
                    x: w - 1 - k,
                    y: h - 1,
                },
                Direction::West,
            );
        }
        k -= w - 1;

        (Vector2d { x: 0, y: h - 1 - k }, Direction::South)
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robot_move() {
        let mut robot = Robot::new(6, 3); // Initialize the grid and the robot at (0, 0) facing East.
        robot.step(2); // It moves two steps East to (2, 0), and faces East.
        robot.step(2); // It moves two steps East to (4, 0), and faces East.
        assert_eq!(robot.get_pos(), vec![4, 0]); // return [4, 0]
        assert_eq!(robot.get_dir(), "East"); // return "East"
        robot.step(2); // It moves one step East to (5, 0), and faces East.
        // Moving the next step East would be out of bounds, so it turns and faces North.
        // Then, it moves one step North to (5, 1), and faces North.
        robot.step(1); // It moves one step North to (5, 2), and faces North (not West).
        robot.step(4); // Moving the next step North would be out of bounds, so it turns and faces West.
        // Then, it moves four steps West to (1, 2), and faces West.
        assert_eq!(robot.get_pos(), vec![1, 2]); // return [1, 2]
        assert_eq!(robot.get_dir(), "West"); // return "West"
    }
}
