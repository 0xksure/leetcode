//! Not finished 

pub struct Solution;

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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let uninfected = true;
        let mut start_root;
        let mut not_found_start = true;
        let index = 0;
        while not_found_start {
            start_root = root.as_ref().unwrap().borrow().right.clone();
            index += 1;
            if start_root.as_ref().unwrap().borrow().val == start {
                not_found_start = false;
            }
        }

        // start infecting
        if start_root.as_ref().unwrap().borrow().val == start {}

        return 0;
    }
}
