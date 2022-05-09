fn main() {
    proconio::input! {
        s: String,
    }

    let result = s.chars().filter(|c| *c == '1').count();
    println!("{}", result);
}
