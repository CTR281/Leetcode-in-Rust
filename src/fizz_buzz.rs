use crate::Solution;

impl Solution {
  pub fn fizz_buzz(n: i32) -> Vec<String> {
      (1..=n)
          .fold(Vec::new(), |mut answer, i| {
              match i {
                  i if i % 15 == 0 => answer.push("FizzBuzz".to_string()),
                  i if i % 3 == 0 => answer.push("Fizz".to_string()),
                  i if i % 5 == 0 => answer.push("Buzz".to_string()),
                  i => answer.push(i.to_string()),
              }
              answer
          })
  }

  pub fn fizz_buzz2(n: i32) -> Vec<String> {
      (1..=n)
          .map(|i| match i {
              i if i % 15 == 0 => String::from("FizzBuzz"),
              i if i % 3 == 0 => String::from("Fizz"),
              i if i % 5 == 0 => String::from("Buzz"),
              i => i.to_string()
          })
      .collect()
  }
}
