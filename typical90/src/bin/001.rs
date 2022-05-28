fn binary_search(mid: isize, xs: &[isize], n: usize, l: isize, k: isize) -> bool {
    let mut count = 0;
    let mut start = 0;

    for i in 0..n {
        if xs[i] - start >= mid && mid <= l - xs[i] {
            count += 1;
            start = xs[i];
        }
    }

    if count >= k {
        true
    } else {
        false
    }
}

fn main() {
    proconio::input! {
        n: usize,
        l: isize,
        k: isize,
        mut xs: [isize; n],
    }
    let mut left = -1;
    let mut right = l + 1;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if binary_search(mid, &xs, n, l, k) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
