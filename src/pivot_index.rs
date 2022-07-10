use crate::Solution;

impl Solution {
  pub fn pivot_index1(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return 0;
    }
    let mut pre_sum = 0;
    let mut suf_sum = 0;
    for i in 1..=nums.len() {
        pre_sum += nums[i - 1];
    }
    pre_sum -= nums.last().unwrap();
    if pre_sum == suf_sum {
        return nums.len() as i32 - 1;
    }
    let mut j = nums.len() - 2;
    let mut ans: i32 = -1;
    loop {
        suf_sum += nums[j + 1];
        pre_sum -= nums[j];
        if suf_sum == pre_sum {
            ans = j as i32;
        }
        if j == 0 {
            break ans;
        }
        j -= 1;
    }
  }

  pub fn pivot_index2(nums: Vec<i32>) -> i32 {
      let sum: i32 = nums.iter().sum();
      let mut left_sum = 0;
      for (i, num) in nums.iter().enumerate() {
          if left_sum == sum - left_sum - num {
              return i as i32;
          }
          left_sum += num;
      }
      -1
  }
}