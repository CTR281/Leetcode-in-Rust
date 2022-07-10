use crate::Solution;

impl Solution { // https://leetcode.com/problems/running-sum-of-1d-array/
  pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    res.push(nums[0]);
    for i in 1..nums.len() {
        res.push(res[i - 1] + nums[i]);
    }
    res
  }

  pub fn running_sum2(nums: Vec<i32>) -> Vec<i32> {
    nums
      .into_iter()
      .scan(0, |sum, x| {
        *sum += x;
        Some(*sum)
      })
      .collect()
  }
}
