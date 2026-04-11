//! https://leetcode.com/problems/balanced-binary-tree/
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
fn node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[allow(dead_code)]
pub(crate) struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            if let Some(n) = node {
                let n = n.borrow();

                let left = height(n.left.clone());
                if left == -1 {
                    return -1;
                }

                let right = height(n.right.clone());
                if right == -1 {
                    return -1;
                }

                if (left - right).abs() > 1 {
                    return -1;
                }

                1 + left.max(right)
            } else {
                0
            }
        }

        height(root) != -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_balanced() {
        let root =
            node(1,
                 node(2,
                      node(4, None, None),
                      node(5, None, None)
                 ),
                 node(3,
                      node(6, None, None),
                      None
                 )
            );

        assert_eq!(Solution::is_balanced(root), true);
    }
}
