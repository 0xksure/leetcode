use std::ops::Div;

struct Solution;
impl Solution {
    pub fn sum_base(n: i32, k: i32) -> i32 {
        let mut sum = 0;
        let mut rem = n % k;
        sum += rem;
        let mut div = (n as f64).div(k as f64).floor() as i32;
        while div.ne(&0) {
            rem = div % k;
            div = (div as f64).div(k as f64).floor() as i32;
            sum += rem;
        }
        return sum;
    }
}
