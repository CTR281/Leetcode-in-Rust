use crate::Solution;

impl Solution {
  pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
    use std::collections::{HashMap, BinaryHeap};
    use std::cmp::{Reverse};

    let mut max_heap = words
        .into_iter()
        .fold(HashMap::new(), |mut freqs: HashMap<String, i32>, s| {
            let count = freqs.entry(s).or_insert(0);
            *count += 1;
            freqs
        })
        .into_iter()
        .fold(BinaryHeap::new(), |mut max_heap, (s, c)| {
            max_heap.push((c, Reverse(s)));
            max_heap
        });

    let mut ans = vec![max_heap.pop().unwrap().1.0];
    let mut count = 1;
    loop {
        match max_heap.pop() {
            Some((_, s)) if count < k => {
                ans.push(s.0);
                count += 1;
            },
            _ => break ans,
        }
    }
  }
/*             .sort_unstable_by(|(s1, c1), (s2, c2)| match c1.cmp(c2) {
            Ordering::Equal => s1.cmp(s2),
            o => o
        })
} */

  pub fn top_k_frequent2(words: Vec<String>, k: i32) -> Vec<String> {
    use std::collections::{HashMap, BinaryHeap};
    use std::cmp::{Reverse};

    let mut max_heap: BinaryHeap<(i32, Reverse<String>)> = words
        .into_iter()
        .fold(HashMap::new(), |mut freqs: HashMap<String, i32>, s| {
            *freqs.entry(s).or_insert(0) += 1;
            freqs
        })
        .into_iter()
        .map(|(s, c)| (c, Reverse(s)))
        .collect();
        
    let mut ans = vec![max_heap.pop().unwrap().1.0];
    let mut count = 1;
    while let Some((_, Reverse(s))) = max_heap.pop() {
        if count >= k {break;}
        ans.push(s);
        count += 1;
    }
    ans
  }
}