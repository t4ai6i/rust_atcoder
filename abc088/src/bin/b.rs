use itertools::Itertools;
use num_integer::Integer;

fn main() {
    proconio::input! {
        n: u8,
        xs: [i8; n]
    }

    let result = xs.iter()
        .sorted_by_key(|&&x| -x)
        .enumerate()
        .map(|(i, &x)| {
            match i.is_even() {
                true => x as isize,
                false => -x as isize,
            }
        })
        .sum::<isize>();
    println!("{}", result);
}
