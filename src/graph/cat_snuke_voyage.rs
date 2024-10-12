// https://atcoder.jp/contests/abc068/tasks/arc079_a
#[allow(non_snake_case)]
pub fn main_cat_snuke_and_a_voyage() {
    // N(頂点の数), M(辺の数)を受け取る
    proconio::input! {
        N:usize,
        M:usize
    }

    // 各頂点の隣接リスト生成
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); N];
    for _i in 0..M {
        proconio::input! {
            mut a:usize,
            mut b:usize
        }

        // 0始まりにする
        a -= 1;
        b -= 1;

        graph[a].push(b);
        graph[b].push(a);
    }

    // 頂点1から、頂点Nまで、2便で到達できるか
    // let mut can_reach = false;
    // for p in &graph[0] {
    //     if graph[*p].contains(&(N - 1)) {
    //         can_reach = true;
    //         println!("POSSIBLE");
    //         return;
    //     }
    // }

    // println!("IMPOSSIBLE");
    let can_reach = graph[0].iter().any(|&p| graph[p].contains(&(N - 1)));

    if can_reach {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
