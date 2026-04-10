//! https://leetcode.com/problems/count-complete-tree-nodes/
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

pub fn node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

pub(crate) struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                let node_ref = node.borrow();
                1 + Self::count_nodes(node_ref.left.clone())
                    + Self::count_nodes(node_ref.right.clone())
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_nodes() {
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

        assert_eq!(Solution::count_nodes(root), 6);
    }
}
