use crate::Solution;
use std::collections::HashMap;

impl Solution {
  pub fn makesquare(matchsticks: Vec<i32>) -> bool { // https://leetcode.com/problems/matchsticks-to-square 
    // Time complexity: O(4^N) (each stick will be placed in one of the four edges: "13214...23": 4^N possibilities. 
    // Worst scenario we check all configurations. Sorting in decreasing order and aborting early avoid a lot of backtracking in most of LC testcases.
    fn dfs(matchsticks: &mut Vec<i32>, mut edges: [i32; 4], edge_length: i32, cursor: usize) -> bool {
      if cursor == matchsticks.len() {
        return edges == [edge_length; 4];
      }
      let stick = matchsticks[cursor];
      for k in 0..4 {
        if edges[k] + stick <= edge_length {
          edges[k] += stick;
          if dfs(matchsticks, edges, edge_length, cursor + 1) {
            return true
          }
          edges[k] -= stick;
        }
      }
      false
    }

    let mut matchsticks = matchsticks;
    matchsticks.sort_unstable_by(|a, b| b.cmp(a));

    let perimeter = matchsticks.iter().sum::<i32>();
    if perimeter % 4 != 0 { return false }
    let edge_length: i32 = perimeter / 4;
    let edges = [0; 4];
    dfs(&mut matchsticks, edges, edge_length, 0)
  }

  pub fn makesquare2(matchsticks: Vec<i32>) -> bool { // does not work
    let mut matchsticks = matchsticks;
    matchsticks.sort_unstable_by(|a, b| b.cmp(a));
    let perimeter = matchsticks.iter().sum::<i32>();
    if perimeter % 4 != 0 { return false }
    let edge_length: i32 = perimeter / 4;
    let mut edge = 0;
    for _ in 0..4 {
      for stick in &mut matchsticks {
        if edge + *stick <= edge_length {
          edge += *stick;
          *stick = 0;
          if edge == edge_length { edge = 0 }
        }
      }
    }
    return matchsticks.iter().sum::<i32>() == 0;
  }

  pub fn makesquare3(matchsticks: Vec<i32>) -> bool { // does not work
    let mut matchsticks = matchsticks;
    let mut stick_lengths = HashMap::with_capacity(15);
    let mut perimeter = 0;
    for &stick in &matchsticks {
      let count = stick_lengths.entry(stick).or_insert(0);
      *count += 1;
      perimeter += stick;
    }

    if perimeter % 4 != 0 { return false }
    let edge_length = perimeter / 4;

    matchsticks.sort_unstable_by(|a, b| b.cmp(a));
    matchsticks.dedup(); // remove consecutive repeated elements

    let mut edge = 0;
    for _ in 0..4 {
      let mut iter = matchsticks.iter();
      while edge < edge_length {
        let next_stick = edge_length - edge;
        if let Some(count) = stick_lengths.get_mut(&next_stick) {
          edge += edge_length - edge;
          *count -= 1;
          if *count == 0 {
            stick_lengths.remove(&next_stick);
          }
        } else {
          if let Some(stick) = iter.next() {
            let count = stick_lengths.get_mut(&stick).unwrap();
            if *count > 0 {
              edge += stick;
              *count -= 1;
            }
            if *count == 0 {
              stick_lengths.remove(&stick);
            }
          } else {
            return false;
          }
        }
      }
      if edge > edge_length { return false }
      edge = 0;
    }
    true
  }

  pub fn makesquare4(matchsticks: Vec<i32>) -> bool { // does not work
    fn dfs(matchsticks: &mut Vec<i32>, mut edge: i32, edge_length: i32, mut cursor: usize) -> bool {
      println!("{:?}, {edge}, {cursor}", matchsticks);
      while cursor < matchsticks.len() && (matchsticks[cursor] == 0 || edge + matchsticks[cursor] > edge_length) { cursor += 1 }
      if cursor >= matchsticks.len() { return false }
      edge += matchsticks[cursor];
      if edge == edge_length {
        matchsticks[cursor] = 0;
        return true;
      }
        //println!("{edge}");
        let mut next_cursor = cursor + 1;
      while next_cursor < matchsticks.len() && !dfs(matchsticks, edge, edge_length, next_cursor) {
        //let last_stick = matchsticks[cursor + 1];
        next_cursor += 1;
        //while cursor < matchsticks.len() && matchsticks[cursor] == last_stick { cursor += 1}
      }
      if next_cursor >= matchsticks.len() {
        return false;
      } else {
        matchsticks[cursor] = 0;
        return true;
      }
    }

    let mut matchsticks = matchsticks;
    matchsticks.sort_unstable_by(|a, b| b.cmp(a));

    let perimeter = matchsticks.iter().sum::<i32>();
    if perimeter % 4 != 0 { return false }
    let edge_length: i32 = perimeter / 4;
    let mut ans = true;
    for _ in 0..4 {
      println!("{:?}", matchsticks);
      ans &= dfs(&mut matchsticks, 0, edge_length, 0);
    }
    ans
  }

}