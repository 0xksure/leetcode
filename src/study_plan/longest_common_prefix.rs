pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        // start out with first string as prefix and find which parts match the next
        // string in the array
        let mut prefix = strs[0].clone();
        for s in &strs[1..strs.len()] {
            // find common string part
            prefix = prefix
                .chars()
                .zip(s.chars())
                .fold((String::from(""), true), |(prev, cont), (p, s)| {
                    if cont {
                        if p == s {
                            return (prev + &p.to_string(), true);
                        } else {
                            return (prev, false);
                        }
                    }
                    return (prev, cont);
                })
                .0;
        }

        prefix
    }
}
