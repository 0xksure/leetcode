//! Solution is copied from https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/3384605/rust-2-pass-o-n-runtime-o-1-space/?envType=study-plan&id=level-2&languageTags=rust
//! Need to comment out the methods  
//!
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;
impl Solution {
    fn remove_node(node: &mut Option<Box<ListNode>>, idx: i32, idx_to_remove: i32) {
        if let Some(n) = node {
            if idx_to_remove == 0 {
                // if ther linked list is of len 0
                *node = n.next.take();
            } else if idx == idx_to_remove - 1 {
                // get the index before the one to rm
                if let Some(next) = n.next.as_mut() {
                    n.next = next.next.take();
                } else {
                    node.take();
                }
            } else {
                // move to next node in the linked list
                Self::remove_node(&mut n.next, idx + 1, idx_to_remove);
            }
        }
    }

    // find_last_node gets the last node in the linked list
    fn find_last_node(node: Option<&Box<ListNode>>, last_node_idx: &mut i32) {
        if let Some(node) = node {
            Self::find_last_node(node.next.as_ref(), last_node_idx);
            *last_node_idx += 1;
        }
    }

    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut last_node_idx = 0;
        Self::find_last_node(head.as_ref(), &mut last_node_idx);
        let node_to_rm = last_node_idx - n;
        Self::remove_node(&mut head, 0, node_to_rm);
        head
    }
}
