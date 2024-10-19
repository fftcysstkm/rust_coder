// https://atcoder.jp/contests/ABC247/tasks/abc247_c
fn rec(N: usize) -> String {
    if N == 1 {
        return "1".to_string();
    }
    return rec(N - 1) + " " + &N.to_string() + " " + &rec(N - 1);
}

#[allow(non_snake_case)]
pub fn main() {
    proconio::input! {
        N:usize
    }

    println!("{}", rec(N));
}
