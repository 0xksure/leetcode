pub struct Solution {}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut cum_sum = 0;
        let mut running_sum_vec = Vec::new();
        for num in nums {
            cum_sum += num;
            running_sum_vec.push(cum_sum);
        }

        running_sum_vec
    }
}
