struct Solution;

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        for i in 1..coordinates.len() {
            if (coordinates[i][0] - coordinates[i - 1][0]) * (coordinates[1][1] - coordinates[0][1])
                != (coordinates[i][1] - coordinates[i - 1][1])
                    * (coordinates[1][0] - coordinates[0][0])
            {
                return false;
            }
        }
        true
    }
}
