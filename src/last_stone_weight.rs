use crate::Solution;

impl Solution {
  pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
    use std::collections::BinaryHeap;
/*         let mut max_heap = stones
        .iter()
        .fold(BinaryHeap::new(), |mut max_heap, &x| {
            max_heap.push(x);
            max_heap
        }); */
    let mut max_heap = BinaryHeap::from(stones);
    
    while max_heap.len() > 1 {
        match max_heap.pop().unwrap() - max_heap.pop().unwrap() {
            0 => (),
            x => {
                max_heap.push(x);
            },
        }
    }
/*         if let Some(x) = max_heap.pop() {
        x
    } else {
        0
    } */
    max_heap.pop().unwrap_or(0)
  }

  pub fn last_stone_weight2(stones: Vec<i32>) -> i32 {
      use std::collections::BinaryHeap;
      let mut max_heap = BinaryHeap::from(stones);
      
      loop {
          match (max_heap.pop(), max_heap.pop()) {
              (Some(x), Some(y)) if x - y > 0 => max_heap.push(x - y),
              (Some(x), None) => break x,
              (None, _) => break 0,
              _ => ()
          }
      }
  }
}