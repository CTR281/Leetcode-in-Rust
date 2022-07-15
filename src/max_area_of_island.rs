use crate::Solution;

impl Solution {
  pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 { // https://leetcode.com/problems/max-area-of-island/
      
      fn expand(grid: &mut Vec<Vec<i32>>, mut area: i32 , i: usize, j: usize, m: usize, n: usize) -> i32 {
          if grid[i][j] == 0 { return area; }
          grid[i][j] = 0;
          if i > 0 { area += expand(grid, 0, i - 1, j, m, n);}
          if i < m - 1 { area += expand(grid, 0, i + 1, j, m, n);}
          if j > 0 { area += expand(grid, 0, i, j - 1, m, n);}
          if j < n - 1 { area += expand(grid, 0, i, j + 1, m, n);}
          return area + 1;
      }
      
      let mut grid = grid;
      let mut max_area = 0;
      let (m , n) = (grid.len(), grid[0].len());
      
      for i in 0..m {
          for j in 0..n {
              max_area = max_area.max(expand(&mut grid, 0, i, j, m, n));
          }
      }   
      max_area
  }
}
