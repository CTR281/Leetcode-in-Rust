use crate::Solution;

impl Solution {
  pub fn character_replacement(s: String, k: i32) -> i32 {
    use std::cmp;

    let s = s.as_bytes();
    let mut counter = 0;
    let mut ans = 0;
    for (i, bi) in s.iter().enumerate() {
        let mut substring_length = 0;
        for bj in s.iter().skip(i) {
            if bj != bi {
                if counter >= k {
                    break;
                } else {
                    
                }
                counter += 1;
            }
            substring_length += 1;
        }
        ans = cmp::max(ans, substring_length);
        if i < s.len() - 2 {
            let bi2 = &s[i + 1];
            counter = 0;
            for bj in s.iter().skip(i) {
                if bj != bi2 {
                    if counter >= k {
                        break;
                    } else {

                    }
                    counter += 1;
                }
                substring_length += 1;
            }
            ans = cmp::max(ans, substring_length);
        }
    }
    ans
  }

  pub fn character_replacement2(s: String, k: i32) -> i32 {
    use std::cmp;

    let s = s.as_bytes();
    let mut count = [0; 26];
    let mut max_count = 0i32;
    let mut ans = 0i32;
    let (mut start, end) = (0, 0);
    while end < s.len() {
        count[(s[end] - b'A') as usize] += 1;
        max_count = cmp::max(max_count, count[(s[end] - b'A') as usize]);
        while end as i32 - start as i32 - max_count + 1 > k {
            count[(s[start] - b'A') as usize] -= 1;
            start += 1;
        }
        ans = cmp::max(ans, end as i32 - start as i32 + 1);
    }
    ans
  }
}