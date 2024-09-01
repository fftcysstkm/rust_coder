use std::collections::HashMap;

fn main() {
    proconio::input! {
        n:usize;
    }

    let mut inputs: Vec<String> = Vec::new();

    for _i in 0..n {
        proconio::input! {
            s:string
        }
        inputs.push(s);
    }

    let mut input_maps: Vec<HashMap<string, usize>> = Vec::new();

    // ブラックリストを用意(後で女以外対象のみをフィルタリングする)
    let mut appear_count: HashMap<char, usize> = HashMap::new();

    // 入力ごとに、文字の出現回数をmapに格納
    for s in inputs {
        let mut s_map: HashMap<char, usize> = HashMap::new();
        for ch in s.chars() {
            *s_map.entry(ch).or_insert(0) += 1;
            // ブラックリスト作成用に、出現回数カウント
            appear_count.entry(ch).or_insert(0) += 1;
        }
        input_maps.push(s_map);
    }

    // ブラックリスト作成
    let black_list: Vec<char> = appear_count
        .into_iter()
        .filter(|&(_, value)| value < n)
        .map(|(key, _)| key)
        .collect();

    let min_appear: HashMap<char, usize> = HashMap::new();

    // ブラックリストにふくまれていない、かつより少ない出現回数なら更新
    for s_map in input_maps {
        for (ch, appear) in s_map.iter() {
            if (!black_list.cotains(&ch)) {
                let min_appear_time = *min_appear.get(ch).unwrap_or(&0);
                if (min_appear_time == 0) {
                    min_appear_time.p
                }
            }
        }
    }
}
