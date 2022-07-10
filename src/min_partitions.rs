use crate::Solution;

impl Solution {
  pub fn min_partitions(n: String) -> i32 {
    use std::cmp;

    let mut ans: i32 = 0;
    for c in n.chars() {
        match c {
            '9' => return 9,
            x => ans = cmp::max(ans, x.to_digit(10).unwrap() as i32),
        }
    }
    ans
  }
}