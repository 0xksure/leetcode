pub struct Solution;
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut cashier = [0; 3];

        for i in 0..bills.len() {
            if bills[i].ne(&5) {
                let mut diff = bills[i] - 5;
                println!("initial diff {}, cashier:{:?}", diff, cashier);
                while cashier[2].ne(&0) && (diff - 20).ge(&0) {
                    diff -= 20;
                    cashier[2] -= 1;
                }

                while cashier[1].ne(&0) && (diff - 10).ge(&0) {
                    diff -= 10;
                    cashier[1] -= 1;
                }

                while cashier[0].ne(&0) && (diff - 5).ge(&0) {
                    diff -= 5;
                    cashier[0] -= 1;
                }
                println!("diff for {}: {}, cashier: {:?}", bills[i], diff, cashier);
                if diff != 0 {
                    return false;
                }
            }
            match bills[i] {
                20 => cashier[2] += 1,
                10 => cashier[1] += 1,
                5 => cashier[0] += 1,
                _ => {}
            }
        }
        return true;
    }
}
