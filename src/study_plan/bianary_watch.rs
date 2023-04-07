pub struct Solution;
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        // 0000 000000
        (0i32..1023)
            .map(|bit| (bit.count_ones(), bit >> 6, bit & 0b111111))
            .filter_map(|(ones, h, m)| {
                if ones == (turned_on as u32) && h < 12 && m < 60 {
                    Some(format!("{}:{:02}", h, m))
                } else {
                    None
                }
            })
            .collect()
    }
}
