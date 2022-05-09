use lazy_static::lazy_static;
use proconio::marker::Chars;

lazy_static! {
    static ref TARGET: Vec<Vec<char>> = vec![
        "dream",
        "dreamer",
        "erase",
        "eraser",
    ]
    .iter()
    .map(|s| s.chars().rev().collect())
    .collect();
}

fn accumulator(s: &[char]) -> bool {
    fn accumulator_inner(found: bool, s: &[char]) -> bool {
        if s.len() == 0 {
            return found;
        }
        let found = TARGET.iter().find(|t| {
            s.starts_with(t)
        });
        match found {
            None => false,
            Some(t) => accumulator_inner(true, &s[t.len()..])
        }
    }

    accumulator_inner(false, s)
}

fn main() {
    proconio::input! {
        mut s: Chars,
    }

    s.reverse();
    let result = accumulator(&s[..]);
    let result = if result {
        "YES".to_string()
    } else {
        "NO".to_string()
    };
    println!("{}", result);
}
