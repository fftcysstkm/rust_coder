fn main() {
    proconio::input! {
        n:i64,
        y:i64
    }

    let mut count_ichi = -1;
    let mut count_gosen = -1;
    let mut count_sen = -1;
    for a in 0..n {
        for b in 0..n {
            if y == 10000 * a + 5000 * b + (n - a - b) * 1000 {
                count_ichi = a;
                count_gosen = b;
                count_sen = n - a - b;
            }
        }
    }
    println!("{} {} {}", count_ichi, count_gosen, count_sen);
}
