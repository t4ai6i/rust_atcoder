use itertools::Itertools;

fn main() {
    proconio::input! {
        n: u8,
        xs: [u8; n],
    }
    let result = xs.into_iter().unique().count();
    println!("{}", result);
}
