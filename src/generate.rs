use crate::Solution;

impl Solution { // https://leetcode.com/problems/pascals-triangle/
  pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    (0..num_rows as usize)
    .fold(Vec::with_capacity(num_rows as usize), |mut triangle: Vec<Vec<i32>>, i| {
      triangle.push(
        (0..=i)
        .fold(Vec::with_capacity(i + 1), |mut row: Vec<i32>, x| {
          match x {
            j if j == 0 || j == i => row.push(1),
            j => row.push(triangle[i - 1][j - 1] + triangle[i - 1][j])
          }
          row
        })
      );
      triangle
      }
    )
  }
}
