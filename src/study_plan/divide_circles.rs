use std::ops::Div;

pub struct Solution;
impl Solution {
    pub fn number_of_cuts(n: i32) -> i32 {
        let one = n.div(2);
        if n % 2 != 0 {
            return one + 1;
        }
        return one;
    }
}
