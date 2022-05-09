fn accumulator(xs: Vec<u32>) -> u16 {
    fn accumulator_inner(acc: u16, xs: Vec<u32>) -> u16 {
        use itertools::Itertools;

        match xs.iter().all(num_integer::Integer::is_even) {
            true => {
                let xs = xs.iter().map(|&x| x / 2).collect_vec();
                accumulator_inner(acc + 1, xs)
            },
            false => acc
        }
    }

    accumulator_inner(0, xs)
}

fn main() {
    proconio::input! {
        n: u16,
        xs: [u32; n]
    }

    let result = accumulator(xs);
    println!("{}", result);
}
