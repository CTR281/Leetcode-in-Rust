use crate::Solution;

impl Solution {
  pub fn is_isomorphic(s: String, t: String) -> bool {
    use std::collections::HashMap;

    let mut s_to_t: HashMap<u8, u8> = HashMap::new();
    let mut t_to_s: HashMap<u8, u8> = HashMap::new();
    let t_bytes = t.into_bytes();
    for (i, c) in s.bytes().enumerate() {
        if s_to_t.contains_key(&c) && s_to_t[&c] != t_bytes[i] {
            return false;
        }
        if t_to_s.contains_key(&t_bytes[i]) && t_to_s[&t_bytes[i]] != c {
            return false;
        }
        s_to_t.insert(c, t_bytes[i]);
        t_to_s.insert(t_bytes[i], c);
    }
    return true;
  }

  pub fn is_isomorphic2(s: String, t: String) -> bool {
      let mut s_dict = [0; 256];
      let mut t_dict = [0; 256];
      
      s.chars().zip(t.chars()).all(|(s, t)| {
          let same = s_dict[s as usize] == t_dict[t as usize];
          println!("{}", s as usize - 'a' as usize);
          s_dict[s as usize] = s as u8;
          t_dict[t as usize] = s as u8;
          same
      })
  }
}