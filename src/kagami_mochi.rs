use std::collections::HashSet;

fn main() {
    proconio::input! {
        n:usize
    }

    let mut set = HashSet::new();

    for _i in 0..n {
        proconio::input! {
            num : usize
        }
        set.insert(num);
    }

    println!("{}", set.len());
}
