fn sum(n: usize) -> usize {
    // 終了条件
    if n == 0 {
        return 0;
    }

    // 再帰呼び出し
    return n + sum(n - 1);
}

pub fn main() {
    println!("{}", sum(2));
    println!("{}", sum(3));
    println!("{}", sum(10));
}
