use crate::Solution;

impl Solution {
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
                if top <= dec_count {
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
}