use std::collections::HashSet;

fn main() {
    proconio::input! {
        n:usize,
        s:String
    }

    let mut max_count = 0;

    // 1~N-1の位置で分割
    for i in 1..n {
        // 分割したxとyをそれぞれユニークな配列にする
        let (x_split, y_split) = s.split_at(i);
        let x_chars = unique_chars(x_split);
        let y_chars = unique_chars(y_split);

        let mut tmp_max = 0;
        for x in &x_chars {
            for y in &y_chars {
                if x == y {
                    tmp_max += 1
                }
            }
        }
        if tmp_max > max_count {
            max_count = tmp_max;
        }
    }
    println!("{}", max_count);
}

// ユニークなcharのset
fn unique_chars(str: &str) -> HashSet<char> {
    str.chars().collect()
}
