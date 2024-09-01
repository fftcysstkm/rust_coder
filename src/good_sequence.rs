use std::collections::HashMap;

fn main() {
    proconio::input! {
        n:usize,
    }

    let mut a_map: HashMap<usize, usize> = HashMap::new();
    for _i in 0..n {
        proconio::input! {
            a:usize
        }
        *a_map.entry(a).or_insert(0) += 1;
    }

    let mut remove_count = 0;
    for (key, value) in a_map.iter() {
        // keyのほうが大きいなら、valueの数分、keyをすべて取り除く
        remove_count += if key > value {
            *value
        } else {
            // valueのほうが大きいならvalue - keyの数だけkeyを取り除く
            // keyとvalueが等しいときも0で影響無いのでOK
            *value - *key
        }
    }
    print!("{}", remove_count);
}
