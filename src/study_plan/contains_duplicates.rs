use core::num;
use std::cmp::min;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        for i in 0..nums.len() {
            println!("i+k:{}", i + k as usize);
            for j in (i + 1)..min(i + k as usize, nums.len()) {
                println!(
                    "i: {}, j:{}, nums[i]: {}, nums[j]: {}",
                    i, j, nums[i], nums[j]
                );
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        return false;
    }
}
