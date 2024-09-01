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

    let remove_count: usize = a_map
        .iter()
        .map(|(&key, &value)| {
            if key > value {
                value
            } else {
                value.saturating_sub(key)
            }
        })
        .sum();
    print!("{}", remove_count);
}
