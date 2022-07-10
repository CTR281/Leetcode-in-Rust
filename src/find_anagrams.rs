use crate::Solution;

impl Solution {
  pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    use std::collections::HashMap;

    pub fn is_anagram(s_map: &HashMap<char, i16> , p_map: &HashMap<char, i16> ) -> bool {
        for (key, val) in p_map {
            match s_map.get(key) {
                Some(x) if x == val => continue,
                _ => return false,
            };
        }
        true
    }

    if s.len() < p.len() {return Vec::new()}
    let mut p_map: HashMap<char, i16> = HashMap::new();
    for (_, c) in p.chars().enumerate() {
        let char_count = p_map.entry(c).or_insert(0);
        *char_count += 1;
    }
    let mut ans = Vec::new();
    let mut s_map: HashMap<char, i16> = HashMap::new();
    let s_chars: Vec<char> = s.chars().collect();
    for c in &s_chars[..p.len()] {
        let char_count = s_map.entry(*c).or_insert(0);
        *char_count += 1;
    }
    if let true = is_anagram(&s_map, &p_map) {
        ans.push(0);
    }
    for (i, c) in s_chars.iter().enumerate().skip(p.len()) {
        let char_count = s_map.entry(*c).or_insert(0);
        *char_count += 1;
        if let Some(x) = s_map.get_mut(&s_chars[i - p.len()]) {
           if *x > 0 {
            *x -= 1;
           }
        }
        if is_anagram(&s_map, &p_map) == true {
            ans.push((i - p.len() + 1) as i32);
        }
    }
    ans
  }

  pub fn find_anagrams2(s: String, p: String) -> Vec<i32> {
    if p.len() > s.len() { return Vec::new() }
    
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let mut s_freqs = [0u8; 26];
    let mut p_freqs = [0u8; 26];   
    for b in p {
        p_freqs[(b - b'a') as usize] += 1;
    }
    for b in &s[..p.len()] {
        s_freqs[(b - b'a') as usize] += 1;
    }

    let mut ans = Vec::new();
    if s_freqs == p_freqs {
        ans.push(0i32);
    } 

    for (i, b) in s.iter().enumerate().skip(p.len()) {
        s_freqs[(b - b'a') as usize] += 1;
        s_freqs[(s[i - p.len() as usize] - b'a') as usize] -= 1;
        if s_freqs == p_freqs {
            ans.push((i - p.len() + 1) as i32);
        }
    } 
    ans
  }
}