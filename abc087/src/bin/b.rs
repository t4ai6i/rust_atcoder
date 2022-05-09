use itertools::iproduct;

fn main() {
    proconio::input! {
        a: u16,
        b: u16,
        c: u16,
        x: u16,
    }

    let result = iproduct!(0..=a, 0..=b, 0..=c)
        .filter_map(|(a, b, c)| {
            match 500 * a + 100 * b + 50 * c {
                sum if sum == x => Some(()),
                _ => None,
            }
        })
        .count();

    println!("{}", result);
}
