pub struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let num_cols = grid.get(0).unwrap().len();
        let num_rows = grid.len();
        let mut i = 0; // col counter
        let mut j = 0; // row counter
        let mut sol: Vec<i32> = vec![1; num_cols];
        for col in 0..(num_cols) {
            // try drop ball
            // continue if grid[i][j] == grid[i][j+1]
            i = col;
            j = 0;
            while j < num_rows {
                let current_value = grid.get(j).unwrap().get(i).unwrap();
                if current_value == &1 {
                    let right_value = match grid.get(j).unwrap().get(i + 1) {
                        Some(val) => val,
                        None => {
                            sol[col] = -1;
                            j = num_rows;
                            continue;
                        }
                    };
                    if right_value != &1 {
                        sol[col] = -1;
                        j = num_rows;
                        continue;
                    }
                }

                if current_value == &-1 {
                    let right_value = match grid.get(j).unwrap().get(i - 1) {
                        Some(val) => val,
                        None => {
                            sol[col] = -1;
                            j = num_rows;
                            continue;
                        }
                    };
                    if right_value != &-1 {
                        sol[col] = -1;
                        j = num_rows;
                        continue;
                    }
                }

                if grid.get(j).unwrap().get(i).unwrap() == &1 {
                    j += 1;
                    i += 1
                } else {
                    j += 1;
                    i -= 1;
                }
            }

            if sol[col].ne(&-1) {
                sol[col] = i as i32;
            }
        }
        return sol;
    }
}
