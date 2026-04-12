//! https://leetcode.com/problems/implement-stack-using-queues/description/
struct MyStack {
    q: Vec<i32>,
}

#[allow(dead_code)]
impl MyStack {
    fn new() -> Self {
        Self { q: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.q.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.q.pop().unwrap()
    }

    fn top(&self) -> i32 {
        self.q.last().unwrap().clone()
    }

    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_stack() {
        let mut stack = MyStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.top(), 2);
        assert_eq!(stack.pop(), 2);
        assert_eq!(stack.empty(), false);
    }
}
