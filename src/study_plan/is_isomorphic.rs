use std::vec;

pub struct Solution {}

// NOT COMPLETED!!

/**
* Given two strings s and t, determine if they are isomorphic.

   Two strings s and t are isomorphic if the characters in s can be replaced to get t.

   All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

   Solution
   __________________________________________
   use hash map: <key,value> to track mapping
   ex: egg -> add
   1. hash map emtpy -> [{e:a}]
   2. check g in hash map -> non -> add [{e:a},{g:d}]
   3. check g in hash map -> d
   true
*/
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        // let string_len = s.len();
        // if (string_len != t.len()) {
        //     return false;
        // }

        // let mut word_map: Vec<u32> = vec![0; 10000];
        // let mut s_chars = s.chars();
        // let mut t_chars = t.chars();
        // for i in 0..s.chars().count() {
        //     let s_char: u32 = s_chars.next().unwrap().into();
        //     let t_char: u32 = t_chars.next().unwrap().into();
        //     let word_map_char = word_map.get(s_char as usize);
        //     println!("s={}, t={}", s_char, t_char);
        //     match word_map_char {
        //         Some(word) => {
        //             if !word.eq(&t_char) {
        //                 return false;
        //             }
        //         }
        //         None => match word_map.iter().find(|&&x| x == s_char) {
        //             Some(val) => {
        //                 println!("match {}", o);
        //                 return val.eq(other);
        //             }
        //             None => {}
        //         },
        //     }
        //     word_map.insert(s_char as usize, t_char);
        // }

        return true;
    }
}
