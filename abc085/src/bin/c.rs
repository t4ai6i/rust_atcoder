fn main() {
    proconio::input! {
        n: i16,
        y: u32,
    }

    let result = (0..=n).find_map(|a| {
        (0..=n - a).find_map(|b| {
            let c = n - a - b;
            let sum = 10_000 * a as u32 + 5_000 * b as u32 + 1_000 * c as u32;
            if sum == y { Some((a, b, c)) } else { None }
        })
    }).unwrap_or((-1, -1, -1));

    println!("{} {} {}", result.0, result.1, result.2);
}
