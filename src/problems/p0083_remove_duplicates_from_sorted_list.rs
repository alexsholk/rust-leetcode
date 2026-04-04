//! https://leetcode.com/problems/remove-duplicates-from-sorted-list/
pub(crate) struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;

        for &val in values.iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }

        head
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = &mut head;

        while let Some(node) = current {
            while node.next.is_some() && node.next.as_ref().unwrap().val == node.val {
                let next_next = node.next.as_mut().unwrap().next.take();
                node.next = next_next;
            }

            current = &mut node.next;
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates() {
        let list = ListNode::from_vec(vec![1, 2, 2, 3, 4, 4, 5]);
        let expected = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::delete_duplicates(list), expected);
    }
}
