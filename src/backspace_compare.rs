use crate::Solution;

impl Solution {
  pub fn backspace_compare(s: String, t: String) -> bool {
    let build = |s: String| {
        s
        .chars()
        .fold(String::new(), |mut new_s: String, c| {
            if let '#' = c {
                new_s.pop();
            } else {
                new_s.push(c);
            }
            new_s
        })
    };
    build(s) == build(t)
  }
}