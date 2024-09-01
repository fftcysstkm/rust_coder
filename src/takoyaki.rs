use proconio::input;

fn main() {
    input! {
        n:i32,
        x:i32,
        t:i32
    }
    let times = if n % x > 0 { n / x + 1 } else { n / x };
    println!("{}", times * t);
}
