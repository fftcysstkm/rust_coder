fn main() {
    proconio::input! {
        n:usize,
        k:usize,
    }

    // 全部0のバケット用意
    let mut a_list = [0; 200000];
    for _ in 0..n {
        proconio::input! {
            a:usize
        }
        a_list[a] += 1;
    }

    // 値が1以上の要素のみ取得
    let mut exist_list: Vec<usize> = a_list.iter().filter(|&&x| x > 0).cloned().collect();

    // 対象のリストがk種類未満だったら終了
    if exist_list.len() < k {
        println!("{}", 0);
        return;
    }

    // 小さい順にソート
    exist_list.sort();

    // リストサイズ-K個の要素を合計
    let mut result: usize = 0;
    for i in 0..(exist_list.len() - k) {
        result += exist_list[i];
    }
    println!("{}", result);
}
