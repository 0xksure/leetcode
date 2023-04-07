pub struct Solution;
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        // stream [1,n]
        let mut j = 0;
        let mut commands = Vec::new();
        for i in 1..n {
            if j == target.len() {
                break;
            }
            println!("i: {}, j: {},target:{}", i, j, target[j]);
            if target[j] == i {
                commands.push("Push".to_string());
                j += 1;
            } else {
                commands.push("Push".to_string());
                commands.push("Pop".to_string());
            }
        }
        commands
    }
}
