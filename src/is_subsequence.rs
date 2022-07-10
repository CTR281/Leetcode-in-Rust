use crate::Solution;

impl Solution {
  pub fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    let s  = s.into_bytes();
    let t = t.into_bytes();
    loop {
        if j >= t.len() {
            break false;
        }
        if s[i] == t[j] {
            i += 1;
        }
        if i == s.len() {
            break true;
        }
        j += 1;
    }
  }
}