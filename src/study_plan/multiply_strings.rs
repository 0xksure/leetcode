//! Not finished!

use std::{fmt::format, ops::Mul};

pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // we remember that mulitplication goes left to right
        // e.g. 123 x 456  = 100x(456) + 20x(456) + 3x(456)
        //                 = 56088
        // a strategy would be to first
        // 1. 3*6, 2*6, 1*6 and carry the tens
        let prod = num2
            .chars()
            .rev()
            .map(|n2| {
                return num1
                    .chars()
                    .rev()
                    .fold((String::from(""), 0), |(prod, carry), n1| {
                        let n1i32 = n1.to_digit(10).unwrap();
                        let n2i32 = n2.to_digit(10).unwrap();
                        let pp = n1i32.mul(n2i32) + carry;
                        let new_carry = pp / 10;
                        let prod = format!("{}{}", pp % 10, prod);

                        return (prod, new_carry);
                    });
            })
            .fold(
                (String::from(""), String::from("")),
                |(prev, mult), (nxt, _)| {
                    let mut n1 = prev;
                    let mut n2 = format!("{}{}", nxt, mult);
                    let n1_len = n1.clone().len();
                    if n2.len() >= n1.len() {
                        n1 = format!("{}{}", vec!["0"; n2.len() - n1_len].join(""), n1.clone());
                    } else {
                        n2 = format!("{}{}", vec!["0"; n1_len - n2.len()].join(""), n2.clone());
                    }
                    let res = n2.chars().rev().zip(n1.chars().rev()).fold(
                        (String::from(""), 0),
                        |(prev, carry), n| {
                            let n1i32 = n.0.to_digit(10).unwrap();
                            let n2i32 = n.1.to_digit(10).unwrap();
                            let ss = n1i32 + n2i32 + carry;
                            let new_carry = ss / 10;
                            let ss = format!("{}{}", ss % 10, prev);
                            return (ss, new_carry);
                        },
                    );
                    println!("res: {:?}, n1: {}, n2: {}", res, n1, n2);

                    return (res.0, format!("0{}", mult,));
                },
            );
        println!("{:?}: ", prod);

        return String::from("");
    }
}
