use num_integer::Integer;

fn main() {
    proconio::input! {
        a: u16,
        b: u16,
    }

    println!("{}", if (a * b).is_even() { "Even" } else { "Odd" })
}
