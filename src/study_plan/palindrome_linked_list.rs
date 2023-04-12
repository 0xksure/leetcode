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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut node = head;
        let mut values = Vec::new();
        while node.is_some() {
            let next_node = node.unwrap();
            values.push(next_node.val);
            node = next_node.next;
        }
        if values.len() == 1 {
            return true;
        }

        let mut j = values.len() - 1;
        for i in 0..(values.len() / 2) {
            if values[i] != values[j] {
                return false;
            }
            j -= 1;
        }
        return true;
    }
}
