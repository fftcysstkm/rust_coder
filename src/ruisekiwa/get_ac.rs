// 累積和問題
//https://atcoder.jp/contests/abc122/tasks/abc122_c
#[allow(non_snake_case)]
pub fn main_get_ac() {
    // 文字列長さ、クエリ数、文字列
    proconio::input! {
        N:usize,
        Q:usize,
        S:String
    }

    // "AC"が登場する回数の累積和配列
    let mut ruiseki: Vec<usize> = vec![0; N + 1];
    for i in 1..N {
        // 一つ前の文字と現在の文字がACなら加算
        let add_num: usize = if &S[i - 1..i + 1] == "AC" { 1 } else { 0 };
        ruiseki[i] = ruiseki[i - 1] + add_num;
    }

    // print!("{:#?}", ruiseki);

    // 各クエリの、区間の出現回数を計算
    for _i in 0..Q {
        // 区間指定を受け取る
        proconio::input! {
            mut l:usize,
            mut r:usize
        }

        // 区間のACの出現回数
        println!("{}", ruiseki[r - 1] - ruiseki[l - 1]);
    }
}
