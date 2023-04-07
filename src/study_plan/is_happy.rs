use std::ops::Add;

pub struct Solution;

impl Solution {
    pub fn sum_digits(n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let rem = n % 10;
            res += rem * rem;
            n /= 10;
        }
        res
    }
    pub fn is_happy(n: i32) -> bool {
        let mut sol = Solution::sum_digits(n);
        while sol != 1 {
            sol = Solution::sum_digits(sol);
        }

        return sol == 1;
    }
}
