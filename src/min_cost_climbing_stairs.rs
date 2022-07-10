use crate::Solution;

impl Solution { // https://leetcode.com/problems/min-cost-climbing-stairs/submissions/
  pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
      let n = cost.len();
      let mut dp = vec![0; n];
      dp[0] = cost[0];
      dp[1] = cost[1];
      let mut i = 2;
      while i < n {
          dp[i] = dp[i - 1].min(dp[i - 2]) + cost[i];
          i += 1;
      }
      dp[n - 1].min(dp[n - 2])
  }
  
  pub fn min_cost_climbing_stairs2(cost: Vec<i32>) -> i32 {
    cost
        .into_iter()         
        .fold([0, 0], |acc, x| {
          [acc[0].min(acc[1]) + x, acc[0]]
    })
    .into_iter()
    .min()
    .unwrap()
  }
}
