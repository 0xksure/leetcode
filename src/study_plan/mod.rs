pub mod add_strings;
pub mod array_with_stack;
pub mod bianary_watch;
pub mod check_straight_line;
pub mod contains_duplicates;
pub mod divide_circles;
pub mod find_ball;
pub mod find_numbers;
pub mod find_pivot_index;
pub mod get_min_swaps;
pub mod infected_binary_tree;
pub mod is_happy;
pub mod is_isomorphic;
pub mod is_ugly;
pub mod lemonade_change;
pub mod map_sum;
pub mod merge_two_sorted_lists;
pub mod min_ascii;
pub mod remove_duplicates;
pub mod replace_words;
pub mod running_sums;
pub mod solitare;
pub mod spiral_matrix;
pub mod sum_base;
pub mod valid_parentheses;
pub fn run() {
    let bills = [5, 5, 5, 5, 10, 5, 10, 10, 10, 20].to_vec();
    lemonade_change::Solution::lemonade_change(bills);
}
