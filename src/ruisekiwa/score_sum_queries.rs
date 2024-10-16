// https://atcoder.jp/contests/typical90/tasks/typical90_j
// 累積和問題、競プロ典型90問
#[allow(non_snake_case)]
pub fn main() {
    // 生徒数
    // 生徒の配列(組、点数)
    proconio::input! {
        N:usize,
        students: [(usize, usize); N]
    }
    // println!("{:#?}", students);

    // 組別の累積和ベクタを初期化
    let mut s_1: Vec<usize> = vec![0; N + 1];
    let mut s_2: Vec<usize> = vec![0; N + 1];

    // 生徒のリストから、組別に累積和生成
    for i in 1..(N + 1) {
        s_1[i] = s_1[i - 1];
        s_2[i] = s_2[i - 1];
        if students[i - 1].0 == 1 {
            s_1[i] += students[i - 1].1
        } else {
            s_2[i] += students[i - 1].1
        }
    }

    // クエリをQ個受取り、総和の区間値出力
    proconio::input! {
        Q:usize,
        queries:[(usize, usize); Q]
    }

    queries.iter().for_each(|q: &(usize, usize)| {
        println!("{} {}", s_1[q.1] - s_1[q.0 - 1], s_2[q.1] - s_2[q.0 - 1])
    });
}
