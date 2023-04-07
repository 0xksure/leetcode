pub struct Solution;
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut spiral = Vec::new();
        let mut horisontal_search = true;
        let mut reversed = false;
        let mut i = 0;
        let mut j = 0;

        let n = matrix.get(0).unwrap().len();
        let m = matrix.len();
        let mut cols = matrix.get(0).unwrap().len();
        let mut rows = matrix.len();
        let mut start_col = 0;
        let mut start_row = 0;
        while spiral.len() < n * m {
            // move horisontally
            println!(
                "i: {}, j:{}, nxm: {}x{}, start_row: {}, start_col: {}",
                i, j, n, m, start_row, start_col
            );
            if horisontal_search {
                spiral.push(*matrix.get(j).unwrap().get(i).unwrap());

                // if reach last column -> move down
                if i == (cols - 1) && !reversed {
                    horisontal_search = false;
                    reversed = false;
                    cols -= 1;
                    j = start_row + 1;
                    continue;
                }
                // if reach first col -> move upwards
                if i == start_col && reversed {
                    horisontal_search = false;
                    reversed = true;
                    start_row += 1;
                    j = rows - 1;
                    continue;
                }

                if reversed {
                    i = start_col.max(i - 1);
                } else {
                    i = cols.min(i + 1)
                }
            }

            // move vertically
            if !horisontal_search {
                spiral.push(*matrix.get(j).unwrap().get(i).unwrap());

                // if reached last row -> search reversed horizontaly
                if j == (rows - 1) && !reversed {
                    horisontal_search = true;
                    reversed = true;
                    rows -= 1;
                    i = cols - 1;
                    continue;
                }

                // if reached first row -> search horisontally right
                if j == start_row && reversed {
                    horisontal_search = true;
                    reversed = false;
                    start_col += 1;
                    i = start_col;
                    continue;
                }

                if reversed {
                    j = start_row.max(j - 1);
                } else {
                    j = rows.min(j + 1)
                }
            }
        }
        return spiral;
    }
}
