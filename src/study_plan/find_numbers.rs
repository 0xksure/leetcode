use core::num;

struct Solution;
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut num_even = 0;
        for i in 0..nums.len() {
            let mut dec = 0;
            let mut num = nums[i];
            while num != 0 {
                num /= 10;
                dec += 1;
            }
            if dec % 2 == 0 {
                num_even += 1
            }
        }

        num_even
    }
}
