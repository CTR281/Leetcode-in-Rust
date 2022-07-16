use crate::Solution;

impl Solution {
  pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 { // https://leetcode.com/problems/out-of-boundary-paths/
      let (m, n, max_move) = (m as usize, n as usize, max_move as usize);
      let mut memo = vec![vec![vec![-1; max_move + 1]; n]; m];

      fn brute(m: usize, n: usize, max_move: usize, i: usize, j: usize, memo: &mut Vec<Vec<Vec<i32>>>) -> i32 {
          if memo[i][j][max_move] != -1 { return memo[i][j][max_move] }
          if max_move == 0 { return 0 }
          memo[i][j][max_move] = 
          (
            (
                  if i < m - 1 { brute(m, n, max_move - 1, i + 1, j, memo) } else { 1 }
                + if i > 0 { brute(m, n, max_move - 1, i - 1, j, memo) } else { 1 }
            ) % 1_000_000_007
            +
            (
                  if j < n - 1 { brute(m, n, max_move - 1, i, j + 1, memo) } else { 1 }
                + if j > 0 { brute(m, n, max_move - 1, i, j - 1, memo) } else { 1 }
            ) % 1_000_000_007
          ) % 1_000_000_007;
          memo[i][j][max_move]
      }

      brute(m, n, max_move, start_row as usize, start_column as usize, &mut memo)
  }
}
