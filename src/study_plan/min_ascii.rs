

pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut s1_chars: Vec<u8> = s1.as_bytes().to_vec();
        s1_chars.sort();
        let mut s2_chars: Vec<u8> = s2.as_bytes().to_vec();
        s2_chars.sort();
        let mut diff_vec = Vec::new();
        for i in 0..s1_chars.len() {
            if !s2_chars.contains(&s1_chars[i]) && !diff_vec.contains(&s1_chars[i]) {
                diff_vec.push(s1_chars[i])
            }
        }

        for j in 0..s2_chars.len() {
            if !s1_chars.contains(&s2_chars[j]) && !diff_vec.contains(&s2_chars[j]) {
                diff_vec.push(s2_chars[j])
            }
        }

        return diff_vec.iter().fold(0_i32, |acc, d| acc + (*d as i32));
    }
}
