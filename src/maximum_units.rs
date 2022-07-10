use crate::Solution;

impl Solution {
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
}