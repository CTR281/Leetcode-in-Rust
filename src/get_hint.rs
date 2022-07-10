use crate::Solution;

impl Solution {
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
    let mut a = 0;
    let mut b = 0;
    let mut cnts = [0i32; 10];
    for (c1, c2) in secret.bytes().zip(guess.bytes()) {
        if c1 == c2 {
            a += 1;
            continue;
        }
        let i1 = (c1 - b'0') as usize;
        let i2 = (c2 - b'0') as usize;
        if cnts[i1] < 0 {
            b += 1;
        }
        if cnts[i2] > 0 {
            b += 1;
        }
        cnts[i1] += 1;
        cnts[i2] -= 1;
    }

    format!("{}A{}B", a, b)
  }
}