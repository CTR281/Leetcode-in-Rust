use crate::Solution;

impl Solution { // https://leetcode.com/problems/jump-game-vi/
  pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
      use std::collections::BinaryHeap;

      let (n, k) = (nums.len(), k as usize);
      let mut score = nums[n - 1];
      let mut heap = BinaryHeap::with_capacity(n);
      heap.push((score, n - 1));
      for i in (0..(n - 1)).rev() {
          score = nums[i] + loop {
              if let Some(&(value, index)) = heap.peek() {
                if index > i + k {
                  heap.pop();
                  continue;
                }
                break value;
              }
          };
          heap.push((score, i));
      }
      score
  }
}