fn main() {
    proconio::input! {
        n:usize,
        m:usize
    }
    let mut vector: Vec<(usize, usize)> = Vec::new();
    for _i in 0..m {
        proconio::input! {
            s:usize,
            c:usize
        }
        vector.push((s, c));
    }

    // 走査する数字を文字列にする
    for i in 0..1000 {
        let i_str = i.to_string();

        if i_str.len() != n {
            continue;
        }
        let mut foud_count = 0;

        // 検査対象数字が、条件をすべて満たすか
        for pair in &vector {
            if i_str.len() >= pair.0 {
                let target_char = i_str.chars().nth(pair.0 - 1).unwrap();
                if (char::from_digit(pair.1 as u32, 10)).unwrap() == target_char {
                    foud_count += 1;
                }
            }
        }
        // 全ての条件に合致する数字があった
        if foud_count == vector.len() {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
