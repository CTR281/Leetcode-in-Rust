use crate::Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, &x) in nums.iter().enumerate() {
        match map.get(&(target - x)) {
            Some(&j) => return vec![i as i32, j],
            None => map.insert(x, i as i32),
        };
    }
    vec![0]
  }
}