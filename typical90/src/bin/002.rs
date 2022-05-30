fn accumulator(xs: &[i32]) -> bool {
    fn accumulator_inner(prev: i32, xs: &[i32]) -> bool {
        let prev = match xs.get(0) {
            None => return if prev == 0 {
                true
            } else {
                false
            },
            Some(&x) => {
                x + prev
            }
        };
        if prev < 0 {
            return false;
        }
        accumulator_inner(prev, &xs[1..])
    }
    accumulator_inner(0, xs)
}

fn main() {
    proconio::input! {
        n: usize
    }
    let results = (0..(1 << n)).into_iter().map(|bit| {
        let mut v = vec![0; n];
        for i in 0..n {
            let bit_i = n - 1 - i;
            v[bit_i] = if bit & (1 << i) == 0 { 1 } else { -1 };
        }
        v
    }).filter(|xs| {
        accumulator(xs)
    }).map(|xs| {
        xs.iter().map(|&x| {
            match x {
                1 => '(',
                -1 => ')',
                n => panic!("Unexpected value = {}", n),
            }
        }).collect::<Vec<char>>().iter().collect::<String>()
    }).collect::<Vec<String>>();
    for result in results {
        println!("{}", result);
    }
}
