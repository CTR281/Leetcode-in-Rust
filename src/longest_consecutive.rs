use crate::Solution;

impl Solution {
  pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    nums
        .into_iter()
        .fold(HashMap::new(), |mut sequences: HashMap<i32, i32>, num| {
            let (&prev_start, &next_end) = (sequences.get(&(num - 1)).unwrap_or(&0), sequences.get(&(num + 1)).unwrap_or(&0));
            let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
            if !sequences.contains_key(&num) {
                if has_prev && has_next {
                    *sequences.get_mut(&prev_start).unwrap() = next_end;
                    *sequences.get_mut(&next_end).unwrap() = prev_start;
                    sequences.insert(num, num);
                } else {
                    if has_prev {
                        sequences.insert(num, prev_start);
                        *sequences.get_mut(&prev_start).unwrap() = num;
                    }
                    else if has_next {
                        sequences.insert(num, next_end);
                        *sequences.get_mut(&next_end).unwrap() = num;
                    } else {
                        sequences.insert(num, num);
                    }
                }
            }
            sequences
        })
        .into_iter()
        .map(|(start, end)| i32::abs(end - start) + 1)
        .max()
        .unwrap_or(0)
  }

  pub fn longest_consecutive_og(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    nums         
        .into_iter()
        .fold((HashMap::new(), 0), |(mut sequences, ans), num| {
            let (mut prev_start, mut next_end) = (*sequences.get(&(num - 1)).unwrap_or(&0), *sequences.get(&(num + 1)).unwrap_or(&0));
            let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
            if !sequences.contains_key(&num) {
                match (has_prev, has_next) {
                    (true, true) => {
                        *sequences.get_mut(&prev_start).unwrap() = next_end;
                        *sequences.get_mut(&next_end).unwrap() = prev_start;
                        sequences.insert(num, num);
                    },
                    (true, false) => {
                        sequences.insert(num, prev_start);
                        *sequences.get_mut(&prev_start).unwrap() = num;
                        next_end = num;
                    },
                    (false, true) => {
                        sequences.insert(num, next_end);
                        *sequences.get_mut(&next_end).unwrap() = num;
                        prev_start = num;
                    },
                    (false, false) => {sequences.insert(num, num);}
                };
            } else {
                prev_start = next_end;
            }
            (sequences, ans.max(i32::abs(next_end - prev_start) + 1))
        })
        .1
  }

  pub fn longest_consecutive_og_reduced(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let n = nums.len();
    nums    
        .into_iter()
        .fold((HashMap::with_capacity(n), 0), |(mut seq, ans), num| {
            let (prev_start, next_end) = (*seq.get(&(num - 1)).unwrap_or(&num), *seq.get(&(num + 1)).unwrap_or(&num));
            if !seq.contains_key(&num) {
                seq.insert(num, num);
                *seq.get_mut(&prev_start).unwrap() = next_end;
                *seq.get_mut(&next_end).unwrap() = prev_start;
                (seq, ans.max(i32::abs(next_end - prev_start + 1)))
            }
            else {
                (seq, ans.max(1))
            }
        })
        .1
  }

  pub fn longest_consecutive_with_capacity(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let n = nums.len();
    nums         
        .into_iter()
        .fold((HashMap::with_capacity(n), 0), |(mut sequences, ans), num| {
            let (mut prev_start, mut next_end) = (*sequences.get(&(num - 1)).unwrap_or(&0), *sequences.get(&(num + 1)).unwrap_or(&0));
            let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
            if !sequences.contains_key(&num) {
                match (has_prev, has_next) {
                    (true, true) => {
                        *sequences.get_mut(&prev_start).unwrap() = next_end;
                        *sequences.get_mut(&next_end).unwrap() = prev_start;
                        sequences.insert(num, num);
                    },
                    (true, false) => {
                        sequences.insert(num, prev_start);
                        *sequences.get_mut(&prev_start).unwrap() = num;
                        next_end = num;
                    },
                    (false, true) => {
                        sequences.insert(num, next_end);
                        *sequences.get_mut(&next_end).unwrap() = num;
                        prev_start = num;
                    },
                    (false, false) => {sequences.insert(num, num);}
                };
                (sequences, ans.max(i32::abs(next_end - prev_start) + 1))
            } else {
                (sequences, ans.max(1))
            }
        })
        .1
  }

  pub fn longest_consecutive_with_capacity_and_default_hash(nums: Vec<i32>) -> i32 {

    use std::collections::{HashMap, hash_map::DefaultHasher};
    use std::hash::BuildHasherDefault;
    

    let n = nums.len();
    let hash = BuildHasherDefault::<DefaultHasher>::default();
    nums         
        .into_iter()
        .fold((HashMap::with_capacity_and_hasher(n, hash), 0), |(mut sequences, ans), num| {
            let (mut prev_start, mut next_end) = (*sequences.get(&(num - 1)).unwrap_or(&0), *sequences.get(&(num + 1)).unwrap_or(&0));
            let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
            if !sequences.contains_key(&num) {
                match (has_prev, has_next) {
                    (true, true) => {
                        *sequences.get_mut(&prev_start).unwrap() = next_end;
                        *sequences.get_mut(&next_end).unwrap() = prev_start;
                        sequences.insert(num, num);
                    },
                    (true, false) => {
                        sequences.insert(num, prev_start);
                        *sequences.get_mut(&prev_start).unwrap() = num;
                        next_end = num;
                    },
                    (false, true) => {
                        sequences.insert(num, next_end);
                        *sequences.get_mut(&next_end).unwrap() = num;
                        prev_start = num;
                    },
                    (false, false) => {sequences.insert(num, num);}
                };
            } else {
                prev_start = next_end;
            }
            (sequences, ans.max(i32::abs(next_end - prev_start) + 1))
        })
        .1
  }

  pub fn longest_consecutive_with_capacity_and_random_state(nums: Vec<i32>) -> i32 {

    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;
    

    let n = nums.len();
    let hash = RandomState::new();
    nums         
        .into_iter()
        .fold((HashMap::with_capacity_and_hasher(n, hash), 0), |(mut sequences, ans), num| {
            let (mut prev_start, mut next_end) = (*sequences.get(&(num - 1)).unwrap_or(&0), *sequences.get(&(num + 1)).unwrap_or(&0));
            let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
            if !sequences.contains_key(&num) {
                match (has_prev, has_next) {
                    (true, true) => {
                        *sequences.get_mut(&prev_start).unwrap() = next_end;
                        *sequences.get_mut(&next_end).unwrap() = prev_start;
                        sequences.insert(num, num);
                    },
                    (true, false) => {
                        sequences.insert(num, prev_start);
                        *sequences.get_mut(&prev_start).unwrap() = num;
                        next_end = num;
                    },
                    (false, true) => {
                        sequences.insert(num, next_end);
                        *sequences.get_mut(&next_end).unwrap() = num;
                        prev_start = num;
                    },
                    (false, false) => {sequences.insert(num, num);}
                };
            } else {
                prev_start = next_end;
            }
            (sequences, ans.max(i32::abs(next_end - prev_start) + 1))
        })
        .1
  }

  pub fn longest_consecutive_without_capacity_and_random_state(nums: Vec<i32>) -> i32 {

    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;

    let hash = RandomState::new();
    nums         
        .into_iter()
        .fold((HashMap::with_hasher(hash), 0), |(mut sequences, ans), num| {
            let (mut prev_start, mut next_end) = (*sequences.get(&(num - 1)).unwrap_or(&0), *sequences.get(&(num + 1)).unwrap_or(&0));
            let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
            if !sequences.contains_key(&num) {
                match (has_prev, has_next) {
                    (true, true) => {
                        *sequences.get_mut(&prev_start).unwrap() = next_end;
                        *sequences.get_mut(&next_end).unwrap() = prev_start;
                        sequences.insert(num, num);
                    },
                    (true, false) => {
                        sequences.insert(num, prev_start);
                        *sequences.get_mut(&prev_start).unwrap() = num;
                        next_end = num;
                    },
                    (false, true) => {
                        sequences.insert(num, next_end);
                        *sequences.get_mut(&next_end).unwrap() = num;
                        prev_start = num;
                    },
                    (false, false) => {sequences.insert(num, num);}
                };
            } else {
                prev_start = next_end;
            }
            (sequences, ans.max(i32::abs(next_end - prev_start) + 1))
        })
        .1
  }


  pub fn longest_consecutive_ahashmap(nums: Vec<i32>) -> i32 {
    use ahash::AHashMap;

    let mut sequences = AHashMap::with_capacity(nums.len());
    let mut ans = 0;
    for num in nums {
        let (mut prev_start, mut next_end) = (*sequences.get(&(num - 1)).unwrap_or(&0), *sequences.get(&(num + 1)).unwrap_or(&0));
        let (has_prev, has_next) = (sequences.contains_key(&(num - 1)), sequences.contains_key(&(num + 1)));
        if !sequences.contains_key(&num) {
            match (has_prev, has_next) {
                (true, true) => {
                    *sequences.get_mut(&prev_start).unwrap() = next_end;
                    *sequences.get_mut(&next_end).unwrap() = prev_start;
                    sequences.insert(num, num);
                },
                (true, false) => {
                    sequences.insert(num, prev_start);
                    *sequences.get_mut(&prev_start).unwrap() = num;
                    next_end = num;
                },
                (false, true) => {
                    sequences.insert(num, next_end);
                    *sequences.get_mut(&next_end).unwrap() = num;
                    prev_start = num;
                },
                (false, false) => {sequences.insert(num, num);}
            };
        } else {
            prev_start = next_end;
        }
        ans = ans.max(i32::abs(next_end - prev_start) + 1)
    }
    ans
  }

  pub fn longest_consecutive_ahashmap_reduced(nums: Vec<i32>) -> i32 {
    use ahash::AHashMap;

    let n = nums.len();
    nums    
        .into_iter()
        .fold((AHashMap::with_capacity(n), 0), |(mut seq, ans), num| {
            let (prev_start, next_end) = (*seq.get(&(num - 1)).unwrap_or(&num), *seq.get(&(num + 1)).unwrap_or(&num));
            if !seq.contains_key(&num) {
                seq.insert(num, num);
                *seq.get_mut(&prev_start).unwrap() = next_end;
                *seq.get_mut(&next_end).unwrap() = prev_start;
                (seq, ans.max(i32::abs(next_end - prev_start + 1)))
            }
            else {
                (seq, ans.max(1))
            }
        })
        .1
  }

  pub fn longest_consecutive_hashset(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let set: HashSet<i32> = nums.into_iter().collect();
    set
    .iter()
    .fold(0, |mut ans, &num| {
        if !set.contains(&(num - 1)) {
            let mut length = 0;
            let mut next_num = num;
            while set.contains(&next_num) {
                length += 1;
                next_num += 1;
            }
            ans = ans.max(length);
        }
        ans
    })
  }

  pub fn longest_consecutive_ahashset(nums: Vec<i32>) -> i32 {
    use ahash::AHashSet;

    let mut set: AHashSet<i32> = AHashSet::<i32>::with_capacity(nums.len());
    for num in nums {
        set.insert(num);
    }
    set
    .iter()
    .fold(0, |mut ans, &num| {
        if !set.contains(&(num - 1)) {
            let mut length = 0;
            let mut next_num = num;
            while set.contains(&next_num) {
                length += 1;
                next_num += 1;
            }
            ans = ans.max(length);
        }
        ans
    })
  }

  pub fn longest_consecutive_sort(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    }
    let mut nums = nums;
    nums.sort();

    let mut count = 1;
    let mut max = 1;
    for i in 1..nums.len() {
      let diff = nums[i] - nums[i - 1];
      if diff == 1 {
        count += 1;
        if count > max {
          max = count;
        }
      } else if diff != 0 {
        count = 1;
      }
    }  
    max 
  }
}
