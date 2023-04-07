use std::ops::Index;

struct Solution;

impl Solution {
    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let mut wonderful_numbers = Vec::new();
        let mut numm = num.clone();
        println!("num: {}", num);
        let num_i32 = num.parse::<i64>().unwrap();
        for i in (1..num.len()).rev() {
            // swap
            let mut char_vec: Vec<char> = numm.chars().collect();
            let last = numm.chars().nth(i).unwrap();
            let first = numm.chars().nth(i - 1).unwrap();
            char_vec[(i - 1)] = last;
            char_vec[i] = first;
            numm = char_vec.into_iter().collect();

            println!("numm: {}, num: {}", numm, num);
            let numm_i32 = numm.parse::<i64>().unwrap();
            if numm_i32.gt(&num_i32) {
                wonderful_numbers.push(numm_i32);
            }
        }
        wonderful_numbers.sort();
        return wonderful_numbers[k as usize] as i32;
    }
}
