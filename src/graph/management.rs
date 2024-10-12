// グラフを使わなくても解ける問題
// https://atcoder.jp/contests/abc163/tasks/abc163_c
pub fn main_management() {
    // 全社員数
    proconio::input! {
        n:usize,

    }

    // 直属上司の出現頻度記録用(添字は社員番号)
    let mut hist = vec![0; n];

    // 入力：社員番号iの、直属の上司番号記録
    for _i in 1..n {
        proconio::input! {
            mut a:usize
        }

        // 社員番号は0始まりにしておく
        a -= 1;

        hist[a] = hist[a] + 1;
    }

    // 出力：各社員の直属部下出力
    for i in 0..n {
        println!("{}", hist[i]);
    }
}
