use num_integer::Integer;

fn accumulator(x: u32) -> u32 {
    fn accumulator_inner(acc: u32, n: u32) -> u32 {
        let (d, m) = n.div_mod_floor(&10);
        match n == 0 {
            true => acc,
            false => accumulator_inner(acc + m, d)
        }
    }
    accumulator_inner(0, x)
}

fn main() {
    proconio::input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let result = (1..=n)
        .filter_map(|x| {
            let sum = accumulator(x);
            match a <= sum && sum <= b {
                true => Some(x),
                false => None,
            }
        })
        .sum::<u32>();
    println!("{}", result);
}
