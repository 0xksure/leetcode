// use std::rc::{self, Rc};

// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// // see solution https://users.rust-lang.org/t/merging-two-sorted-linked-lists/74089/
// // rust linked lists https://rust-unofficial.github.io/too-many-lists/
// pub struct Solution {}

// impl Solution {
//     pub fn merge_two_lists(
//         list1: Option<Box<ListNode>>,
//         list2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         // copy start node
//         let start = Box::new(ListNode::new(0));
//         let mut tail = &start;
//         let mut headA = list1;
//         let mut headB = list2;
//         loop {
//             let headA_val = match headA.clone() {
//                 Some(node) => node,
//                 None => {
//                     tail.next = headB;
//                     break;
//                 }
//             };

//             let headB_val = match headB.clone() {
//                 Some(node) => node,
//                 None => {
//                     tail.next = headA;
//                     break;
//                 }
//             };

//             if headA_val.val <= headB_val.val {
//                 tail.next = headA;
//                 headA = headA_val.next;
//             } else {
//                 tail.next = headB;
//                 headB = headB_val.next;
//             }
//             tail = match tail.next {
//                 Some(node) => &node,
//                 None => break,
//             };
//         }

//         return start.next;
//     }
// }
