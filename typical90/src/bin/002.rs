fn accumulator(xs: &[char]) -> bool {
    fn accumulator_inner(depth: i32, xs: &[char]) -> bool {
        let depth = match xs.get(0) {
            None => return if depth == 0 {
                true
            } else {
                false
            },
            Some(&x) => {
                if x == '(' { depth + 1 } else { depth - 1 }
            }
        };
        if depth < 0 {
            return false;
        }
        accumulator_inner(depth, &xs[1..])
    }
    accumulator_inner(0, xs)
}

fn main() {
    proconio::input! {
        n: usize
    }
    let results = (0..(1 << n)).into_iter().map(|bit| {
        let mut v = vec!['\0'; n];
        for i in 0..n {
            let bit_i = n - 1 - i;
            v[bit_i] = if bit & (1 << i) == 0 { '(' } else { ')' };
        }
        v
    }).filter(|xs| {
        accumulator(xs)
    }).map(|xs| {
        xs.iter().collect::<String>()
    }).collect::<Vec<String>>();
    for result in results {
        println!("{}", result);
    }
}
