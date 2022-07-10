use crate::Solution;

impl Solution { // https://leetcode.com/problems/interleaving-string/
  pub fn is_interleave(s1: String, s2: String, s3: String) -> bool { // O(2^(m+n)) time
    fn is_interleave_i_j(s1: &Vec<char>, i: usize, s2: &Vec<char>, j: usize, res: Vec<char>, s3: &Vec<char>) -> bool {
        if i == s1.len() && j == s2.len() && res == *s3 {
            return true;
        }

        let mut ans = false;
        if i < s1.len() {
            let mut res = res.clone();
            res.push(s1[i]);
            ans |= is_interleave_i_j(s1, i + 1, s2, j, res, s3);
        }
        if j < s2.len() {
            let mut res = res.clone();
            res.push(s2[j]);
            ans |= is_interleave_i_j(s1, i, s2, j + 1, res, s3);
        }
        ans
    }

    if s3.len() != s1.len() + s2.len() {
        return false;
    }
    return is_interleave_i_j(&s1.chars().collect(), 0, &s2.chars().collect(), 0, Vec::new(), &s3.chars().collect());
  }

  pub fn is_interleave2(s1: String, s2: String, s3: String) -> bool { // top-down memo
    fn is_interleave_i_j(s1: &Vec<char>, i: usize, s2: &Vec<char>, j: usize, s3: &Vec<char>, k: usize, memo: &mut Vec<Vec<i8>>) -> bool {
        if i == s1.len() {
            return s2[j..] == s3[k..];
        }

        if j == s2.len() {
            return s1[i..] == s3[k..];
        }

        if memo[i][j] != -1 {
            return if let 0 = memo[i][j] { false } else { true }
        }
        let mut ans = false;
        if s1[i] == s3[k] && is_interleave_i_j(s1, i + 1, s2, j, s3, k + 1, memo) || s2[j] == s3[k] && is_interleave_i_j(s1, i, s2, j + 1, s3, k + 1, memo) {
            ans = true;
        }
        memo[i][j] = if let true = ans { 1 } else { 0 };
        ans
    }

    if s3.len() != s1.len() + s2.len() {
        return false;
    }
    let mut memo: Vec<Vec<i8>> = vec![vec![-1; s2.len()]; s1.len()];
    let (s1, i, s2, j, k, s3) = (s1.chars().collect::<Vec<char>>(), 0, s2.chars().collect::<Vec<char>>(), 0, 0, s3.chars().collect::<Vec<char>>());
    return is_interleave_i_j(&s1, i, &s2, j, &s3, k, &mut memo);
  }
}
