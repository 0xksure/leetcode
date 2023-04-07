use core::num;

pub struct Solution {}

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (i, num) in nums.iter().enumerate() {
            if left_sum == (sum - left_sum - num) {
                return i as i32;
            }
            left_sum += num
        }

        return -1;
    }
}
