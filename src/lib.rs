

pub struct Solution;

pub struct ShiftXorHasher {
    state: u64,
}

impl std::hash::Hasher for ShiftXorHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.rotate_left(8) ^ u64::from(byte);
        }
    }
    
    fn finish(&self) -> u64 {
        self.state
    }
}

pub struct BuildShiftXorHasher;

impl std::hash::BuildHasher for BuildShiftXorHasher {
    type Hasher = ShiftXorHasher;
    fn build_hasher(&self) -> ShiftXorHasher {
        ShiftXorHasher { state: 0 }
    }
}

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        res.push(nums[0]);
        for i in 1..nums.len() {
            res.push(res[i - 1] + nums[i]);
        }
        res
    }

    pub fn pivot_index1(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut pre_sum = 0;
        let mut suf_sum = 0;
        for i in 1..=nums.len() {
            pre_sum += nums[i - 1];
        }
        pre_sum -= nums.last().unwrap();
        if pre_sum == suf_sum {
            return nums.len() as i32 - 1;
        }
        let mut j = nums.len() - 2;
        let mut ans: i32 = -1;
        loop {
            suf_sum += nums[j + 1];
            pre_sum -= nums[j];
            if suf_sum == pre_sum {
                ans = j as i32;
            }
            if j == 0 {
                break ans;
            }
            j -= 1;
        }
    }

    pub fn pivot_index2(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let mut left_sum = 0;
        for (i, num) in nums.iter().enumerate() {
            if left_sum == sum - left_sum - num {
                return i as i32;
            }
            left_sum += num;
        }
        -1
    }

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
    pub fn min_partitions(n: String) -> i32 {
        use std::cmp;

        let mut ans: i32 = 0;
        for c in n.chars() {
            match c {
                '9' => return 9,
                x => ans = cmp::max(ans, x.to_digit(10).unwrap() as i32),
            }
        }
        ans
    }

    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        use std::cmp::Ordering;

        let mut box_types_cp = box_types.clone(); 
        box_types_cp.sort_unstable_by(| a, b | b[1].cmp(&a[1]));
        let mut ans = 0;
        let mut k: usize = 0;
        let mut remaining_space = truck_size;
        loop {
            ans += match box_types_cp[k][0].cmp(&remaining_space) {
                Ordering::Greater => truck_size * box_types_cp[k][1],
                _ => box_types_cp[k][0] * box_types_cp[k][1],
            };
            remaining_space -= box_types_cp[k][0];
            if remaining_space <= 0 || k > box_types_cp.len(){
                break ans;
            }
            k += 1;
        }
    }

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
            if (i < s.len() - 2) {
                let bi2 = &s[i + 1];
                counter = 0;
                for bj in s.iter().skip(i) {
                    if bj != bi2 {
                        if (counter >= k) {
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
        let (mut start, mut end) = (0, 0);
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

    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        use std::cmp;

        let mut horizontal_cuts = horizontal_cuts.to_owned();
        horizontal_cuts.push(0);
        horizontal_cuts.push(h);
        horizontal_cuts.sort();
        let mut max_height = 0;
        for (i, _) in horizontal_cuts.iter().enumerate().skip(1) {
            max_height = cmp::max(max_height, horizontal_cuts[i] - horizontal_cuts[i - 1]);
        }

        let mut vertical_cuts = vertical_cuts.to_owned();
        vertical_cuts.push(0);
        vertical_cuts.push(w);
        vertical_cuts.sort();
        let mut max_width = 0;
        for (i, _) in vertical_cuts.iter().enumerate().skip(1) {
            max_width = cmp::max(max_width, vertical_cuts[i] - vertical_cuts[i - 1]);
        }
        (((max_height as i64 * max_width as i64)) % 1000000007) as i32
    }

    pub fn max_area2(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal_cuts = horizontal_cuts;
        horizontal_cuts.extend([0, h]);
        horizontal_cuts.sort_unstable();
        let max_height = horizontal_cuts.windows(2).map(|x| x[1] - x[0]).max().unwrap() as i64;

        let mut vertical_cuts = vertical_cuts;
        vertical_cuts.extend([0, w]);
        vertical_cuts.sort_unstable();
        let max_width = vertical_cuts.windows(2).map(|x| x[1] -x[0]).max().unwrap() as i64;
        (max_height * max_width % 1000000007) as i32
    }

    pub fn max_area3(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let max = |mut cuts: Vec<i32>, dim: i32| {
            cuts.extend([0, dim]);
            cuts.sort_unstable();
            cuts.windows(2).map(|x| x[1] - x[0]).max().unwrap() as u64
        };
       (max(horizontal_cuts, w) * max(vertical_cuts, h) % 1_000_000_007) as i32
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut strs: Vec<String> = Vec::new();
        for (i, &x) in nums.iter().enumerate() {
            match map.get(&(target - x)) {
                Some(&j) => return vec![i as i32, j],
                None => map.insert(x, i as i32),
            };
        }
        vec![0]
    }

    pub fn get_hint(secret: String, guess: String) -> String {
        use std::collections::HashMap;
        
        let (secret, guess) = (secret.as_bytes(), guess.as_bytes());
        let mut secret_map: HashMap<u8, u16> = HashMap::new();
        let (mut a, mut b) = (0, 0);
        for (i, &digit) in secret.iter().enumerate() {
            if digit == guess[i] {
                a += 1;
            } else {
                let digit_count = secret_map.entry(digit).or_insert(0);
                *digit_count += 1;
            }
        }
        for (i, &digit) in guess.iter().enumerate() {
            if secret[i] != digit {
                if let Some(x) = secret_map.get_mut(&digit) {
                    b += 1;
                    *x -= 1;
                    if *x == 0 {
                        secret_map.remove(&digit);
                    }
                }
            }
        }
        format!("{}A{}B", a, b)
    }

    pub fn get_hint2(secret: String, guess: String) -> String {
        let mut A = 0;
        let mut B = 0;
        let mut cnts = [0i32; 10];
        for (c1, c2) in secret.bytes().zip(guess.bytes()) {
            if c1 == c2 {
                A += 1;
                continue;
            }
            let i1 = (c1 - b'0') as usize;
            let i2 = (c2 - b'0') as usize;
            if cnts[i1] < 0 {
                B += 1;
            }
            if cnts[i2] > 0 {
                B += 1;
            }
            cnts[i1] += 1;
            cnts[i2] -= 1;
        }

        format!("{}A{}B", A, B)
    }

    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        if nums.len() == 1 {
            return 1;
        }
        let mut ans = 1;
        let mut lastEle = &nums[0];
        let mut cmp = lastEle.cmp(&nums[1]);
        if let Ordering::Equal = cmp {}         
        else {
            lastEle = &nums[1];
            ans += 1;
        }
        for x in nums.iter().skip(2) {
            if x == lastEle {continue;}
            let cmp_next = lastEle.cmp(x);
            if cmp_next != cmp {
                ans += 1;
            }
            lastEle = x;
            cmp = cmp_next;
        }
        ans
    }

    pub fn wiggle_max_length2(nums: Vec<i32>) -> i32 {
        use std::cmp::{self, Ordering};
        nums[1..]
            .iter()
            .fold((nums[0], None, 1), |(prev, next_ord, res), &x| {
                if next_ord.map_or(x != prev, |o| o == x.cmp(&prev)) {
                    (x, Some(prev.cmp(&x)), res + 1)
                } else {
                    (x, next_ord, res)
                }
            })
            .2
    }

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

    /* pub fn decode_string(s: String) -> String {

        fn update_string(s: String, v: Vec<char>, count: i32, n: String) -> (String, Vec<char>, i32, String) {
            if n.is_empty() {
                (s, v, count, n)
            } else {
                if count == 0 {
                    s.push_str(&v.repeat(n.chars().rev().collect::<String>().parse::<usize>().unwrap()).iter().collect::<String>());
                    v.clear();
                    n.clear();
                    (s, v, count, n)
                } else {
                    v.push
                }
            }
        }
        s
        .chars()
        .rev()
        .fold((String::new(), Vec::new(), 0, String::new()), |(mut s, mut v, count, n): (String, Vec<char>, i32, String), c| {
            match c {
                ']' => update_string(s, v, count + 1, n),
                '[' => update_string(s, v, count - 1, n),
                '1'..='9' => {           
                    n.push(c);
                    (s, v, count, n)
                }          //(0..c.to_digit(10).unwrap()).fold(&mut s, |mut acc, _| )
                c => {
                    v.push(c);
                }
            }
            (s, v, count)
        }).0.chars().rev().collect()
    } */
    pub fn decode_string(s: String) -> String {
        s
        .chars()
        .fold((String::new(), String::new(), Vec::new()), |(mut next_string, mut next_integer, mut stack): (String, String, Vec<(i32, String)>), c| {
            match c {
                '0'..='9' => {
                    next_integer.push(c);
                    (next_string, next_integer, stack)
                },
                '[' => {
                    stack.push((next_integer.parse::<i32>().unwrap(), next_string));
                    next_integer.clear();
                    (String::new(), next_integer, stack)
                },
                ']' => {
                    let (last_integer, left_string) = stack.pop().unwrap();
                    let next_string = left_string + &next_string.repeat(last_integer as usize);
                    (next_string, next_integer, stack)
                },
                c => {
                    next_string.push(c);
                    (next_string, next_integer, stack)
                }
            }
        }).0
    }

    pub fn candy(ratings: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        let (res, top, dec_count, prev_ord) = ratings
        .windows(2)
        .map(|x| x[0].cmp(&x[1]))
        .fold((1, 1, 0, None), |(res, top, dec_count, prev_ord): (i32, i32, i32, Option<Ordering>), x| {
           match x {
            Ordering::Greater => {
                (res + dec_count + 1, top, dec_count + 1, Some(Ordering::Greater))
            },
            Ordering::Equal => {
                if prev_ord.map_or(false, |o| o == Ordering::Greater) && top <= dec_count {
                    (res - top + dec_count + 1 + 1, 1, 0, Some(Ordering::Equal))
                } else {
                    (res + 1, 1, 0, Some(Ordering::Equal))
                }
            },
            Ordering::Less => {
                if prev_ord.map_or(false, |o| o == Ordering::Greater){
                    if (top <= dec_count) {
                        (res - top + dec_count + 1 + 2, 2, 0, Some(Ordering::Less))
                    } else {
                        (res + 2, 2, 0, Some(Ordering::Less))
                    }
                } else {
                    (res + top + 1, top + 1, 0, Some(Ordering::Less))
                }
            }
           } 
        });
        return if prev_ord.map_or(false, |o| o == Ordering::Greater) && top <= dec_count {
            res - top + dec_count + 1
        } else {
            res
        }  
    }

    pub fn candy2(ratings: Vec<i32>) -> i32 {
        use std::cmp::Ordering;

        ratings
        .windows(2)
        .map(|x| x[0].cmp(&x[1]))
        .fold((1, 1, 1, 0), |(res, front, top, dec_count): (i32, i32, i32, i32), x| {
           match x {
            Ordering::Greater => {
                if top <= dec_count + 1 {
                    (res + dec_count + 1 + 1, 1, top + 1, dec_count + 1)
                } else {
                    (res + dec_count + 1, 1, top, dec_count + 1)
                }
            },
            Ordering::Equal => {
                (res + 1, 1, 1, 0)
            },
            Ordering::Less => {
                (res + front + 1, front + 1, front + 1, 0)
            }
           } 
        })
        .0
    }

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
                _ => ()
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

    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        use std::collections::{HashMap, BinaryHeap};
        use std::cmp::{Ordering, Reverse};

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
                Some((c, s)) if count < k => {
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
        use std::cmp::{Ordering, Reverse};

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

    pub fn longest_consecutive2(nums: Vec<i32>) -> i32 {
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
                } else {
                    prev_start = next_end;
                }
                (sequences, ans.max(i32::abs(next_end - prev_start) + 1))
            })
            .1
    }

    pub fn longest_consecutive3(nums: Vec<i32>) -> i32 {
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

    pub fn longest_consecutive4(nums: Vec<i32>) -> i32 {
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

    pub fn longest_consecutive5(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let set:HashSet<i32> = HashSet::from_iter(nums);
        
        let longest_streak: i32 = set.iter()
          .map(|x| {
            if !set.contains(&(x - 1)) {
              let (mut current_streak, mut num) = (0, *x);
            
              while set.contains(&num) {
                num += 1;
                current_streak += 1;
              }
  
              current_streak  
            } else {
              0
            }
            
          })
          .max().unwrap_or_default();
        
        longest_streak
    }

    pub fn longest_consecutive6(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut sequences = HashMap::new();
        nums         
            .into_iter()
            .fold(0, |ans, num| {
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
                ans.max(i32::abs(next_end - prev_start) + 1)
            })
    }

    pub fn longest_consecutive7(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::hash_map::RandomState;
        use ahash::AHashMap;
        let s = RandomState::new();

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

    pub fn longest_consecutive_OG(nums: Vec<i32>) -> i32 {
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
                } else {
                    prev_start = next_end;
                }
                (sequences, ans.max(i32::abs(next_end - prev_start) + 1))
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

    pub fn longest_consecutive_with_capacity_and_custom_hash(nums: Vec<i32>) -> i32 {
    
        use std::collections::HashMap;

        let n = nums.len();
        nums         
            .into_iter()
            .fold((HashMap::with_capacity_and_hasher(n, BuildShiftXorHasher), 0), |(mut sequences, ans), num| {
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

    pub fn longest_consecutive_AHASH(nums: Vec<i32>) -> i32 {
        use std::collections::hash_map::RandomState;
        use ahash::AHashMap;
        let s = RandomState::new();

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


}

fn main() {
    //println!("{:#?}", Solution::running_sum(vec![1, 2, 3]))
    let r = "ab".cmp("b");
}
