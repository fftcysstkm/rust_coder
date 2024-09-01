use std::collections::HashMap;

fn main() {
    proconio::input! {
        n:usize,
        s_list:[String;n]
    }

    // 文字列をソートしてマップに格納
    let mut anagrams = HashMap::new();
    for str in &s_list {
        let mut chars: Vec<char> = str.chars().collect();
        chars.sort();
        let s: String = chars.into_iter().collect();
        *anagrams.entry(s).or_insert(0) += 1;
    }

    // 各アナグラムに対し、組み合わせ:nC2を計算して集計
    // マップのvaluesだけあれば良い
    let mut count: i64 = 0;
    for value in anagrams.values() {
        count += value * (value - 1) / 2;
    }
    println!("{}", count);
}
