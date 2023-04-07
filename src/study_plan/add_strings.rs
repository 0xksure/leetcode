//! NOT FINISHED
//! https://leetcode.com/problems/add-strings/description/
pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        // create integer summation

        let n1: String;
        let n2: String;
        let len_num1 = num1.len();
        let len_num2 = num2.len();
        if len_num1 > len_num2 {
            n1 = num1;
            n2 = Vec::from([0..(len_num1 - len_num2)])
                .iter()
                .fold(String::from(""), |acc, _| format!("{}{}", acc, "0"))
                + &num2;
        } else {
            n1 = num2;
            n2 = Vec::from([0..(len_num2 - len_num1)])
                .iter()
                .fold(String::from(""), |acc, _| format!("{}{}", acc, "0"))
                + &num1;
        }
        println!("n1: {} , n2:{}", n1, n2);
        let (sum, carry) = n1.chars().rev().into_iter().zip(n2.chars().rev()).fold(
            (String::from(""), 0_u32),
            |(sum, carry), (n1, n2)| {
                println!("{} {}", n1, n2);
                let ss = n1.to_digit(10).unwrap() + n2.to_digit(10).unwrap() + carry;
                if ss > 9 {
                    return (sum + "0", ss - 9);
                } else {
                    return (sum + &ss.to_string(), 0);
                }
            },
        );
        if carry > 0 {
            return (sum.clone() + &carry.to_string()).chars().rev().collect();
        }
        return sum.chars().rev().collect();
    }
}
