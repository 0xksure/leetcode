//! Not finished
//!
//! TODO:
//! - Rewrite to creating one odd and even linked list
//! - Combine the odd and even list like: odd -> even
//!
//! Based on solution https://leetcode.com/problems/odd-even-linked-list/solutions/2887522/rust-solution-with-explanation-o-n-time-o-1-space/?envType=study-plan&id=level-2&languageTags=rust
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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // intuition
        // split list into two - odd and even
        let mut odd_list = head;
        let mut even_list = odd_list.as_ref().unwrap().next.to_owned();

        // list to be adjusted
        let mut odd_tail = odd_list.as_mut();
        let mut even_tail = even_list.as_mut();

        while even_tail.is_some() {
            let new_odd = even_tail.as_ref().unwrap().next.to_owned();
            let new_even = match new_odd.as_ref() {
                Some(node) => node.next.to_owned(),
                None => None,
            };
            odd_tail.as_mut().unwrap().next = new_odd;
            even_tail.as_mut().unwrap().next = new_even;

            if odd_tail.as_ref().unwrap().next.is_some() {
                odd_tail = odd_tail.unwrap().next.as_mut();
                even_tail = even_tail.unwrap().next.as_mut();
            } else {
                break;
            }
        }

        odd_tail.unwrap().next = even_list;
        odd_list
    }
}
