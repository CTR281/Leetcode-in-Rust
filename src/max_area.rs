use crate::Solution;

impl Solution {
  pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    use std::cmp;

    let mut horizontal_cuts = horizontal_cuts;
    horizontal_cuts.push(0);
    horizontal_cuts.push(h);
    horizontal_cuts.sort_unstable();
    let mut max_height = 0;
    for (i, _) in horizontal_cuts.iter().enumerate().skip(1) {
        max_height = cmp::max(max_height, horizontal_cuts[i] - horizontal_cuts[i - 1]);
    }

    let mut vertical_cuts = vertical_cuts;
    vertical_cuts.push(0);
    vertical_cuts.push(w);
    vertical_cuts.sort_unstable();
    let mut max_width = 0;
    for (i, _) in vertical_cuts.iter().enumerate().skip(1) {
        max_width = cmp::max(max_width, vertical_cuts[i] - vertical_cuts[i - 1]);
    }
    (((max_height as i64 * max_width as i64)) % 1000000007) as i32
  }

  pub fn max_area2(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let mut horizontal_cuts = horizontal_cuts;
    horizontal_cuts.extend([0, h]);
    horizontal_cuts.sort_unstable();
    let max_height = horizontal_cuts.windows(2).map(|x| x[1] - x[0]).max().unwrap() as i64;

    let mut vertical_cuts = vertical_cuts;
    vertical_cuts.extend([0, w]);
    vertical_cuts.sort_unstable();
    let max_width = vertical_cuts.windows(2).map(|x| x[1] -x[0]).max().unwrap() as i64;
    (max_height * max_width % 1000000007) as i32
  }

  pub fn max_area3(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
    let max = |mut cuts: Vec<i32>, dim: i32| {
        cuts.extend([0, dim]);
        cuts.sort_unstable();
        cuts.windows(2).map(|x| x[1] - x[0]).max().unwrap() as u64
    };
   (max(horizontal_cuts, w) * max(vertical_cuts, h) % 1_000_000_007) as i32
  }
}
