use crate::Solution;

impl Solution {
  pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;

    if nums.len() == 1 {
        return 1;
    }
    let mut ans = 1;
    let mut last_ele = &nums[0];
    let mut cmp = last_ele.cmp(&nums[1]);
    if let Ordering::Equal = cmp {}         
    else {
        last_ele = &nums[1];
        ans += 1;
    }
    for x in nums.iter().skip(2) {
        if x == last_ele {continue;}
        let cmp_next = last_ele.cmp(x);
        if cmp_next != cmp {
            ans += 1;
        }
        last_ele = x;
        cmp = cmp_next;
    }
    ans
  }

  pub fn wiggle_max_length2(nums: Vec<i32>) -> i32 {
    nums[1..]
        .iter()
        .fold((nums[0], None, 1), |(prev, next_ord, res), &x| {
            if next_ord.map_or(x != prev, |o| o == x.cmp(&prev)) {
                (x, Some(prev.cmp(&x)), res + 1)
            } else {
                (x, next_ord, res)
            }
        })
        .2
  }
}