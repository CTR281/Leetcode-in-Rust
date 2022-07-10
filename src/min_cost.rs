use crate::Solution;

impl Solution { // https://leetcode.com/problems/paint-house-iii/
  pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
    
    let (m, n, target) = (m as usize, n as usize, target as usize);
    let max_value = 1_000_001;
    let mut dp = vec![vec![vec![max_value; target]; n]; m];
    let mut min_neighbourhood = 0;

    for j in 0..n{
        dp[0][j][0] = if houses[0] == 0 { cost[0][j] } else { if j == houses[0] as usize - 1 { 0 } else { max_value } };
    }
    if houses[0] != 0 {min_neighbourhood += 1}

    for i in 1..m {
        if houses[i] != 0 {
            if houses[i] != houses[i - 1] && houses[i - 1] != 0 { min_neighbourhood += 1;}
            let j = (houses[i] - 1) as usize;
            dp[i][j][0] = dp[i - 1][j][0];
            
            for k in 1..target {
                if k > i { break; }
                let mut min_cost = dp[i - 1][j][k];
                for j2 in (0..n).filter(|&x| x != j) {
                    min_cost = min_cost.min(dp[i - 1][j2][k - 1]);
                }
                dp[i][j][k] = min_cost;
            }
        } else {
            for j in 0..n {
                dp[i][j][0] = dp[i - 1][j][0] + cost[i][j];
                for k in 1..target {  
                    /*  
                    having k neighbourhoods at the ith house can be done by having already k nbhds at the (i - 1)th house and choosing not to introduce a new nbhd,
                    or having k - 1 nbhds at i - 1 and choosing a different color for the ith house.
                    Additionally, we can't have more neighbourhoods than the current number of houses.
                    */
                    if k > i { break; }
                    let mut min_cost = dp[i - 1][j][k] + cost[i][j]; // Not introducing a new neighbourhood
                    for j2 in (0..n).filter(|&x| x != j) {
                        min_cost = min_cost.min(dp[i - 1][j2][k - 1] + cost[i][j]); // if we want to introduce a new neighbourhood, there is m - 1 ways to do it. Let's choose the cheapest one.
                    }
                    dp[i][j][k] = min_cost;
                }
            }
        }
    }
    let mut ans = dp[m - 1][0][target - 1];
    for j in 1..n {
        ans = ans.min(dp[m - 1][j][target - 1]);
    }
    if min_neighbourhood > target { return -1;}
    if ans < max_value { ans } else { -1 }
  }
}
