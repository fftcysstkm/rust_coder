// 無向グラフ問題
// https://atcoder.jp/contests/past202005-open/tasks/past202005_e
pub fn main_sprinkler() {
    proconio::input! {
        // 頂点の数、辺の数、クエリの数
        n:usize,
        m:usize,
        q:usize
    }

    // 各頂点の隣接リスト
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    // M本の辺の両端の頂点を、グラフとして記録
    for _i in 0..m {
        proconio::input! {
            mut u:usize,
            mut v:usize
        }

        // 頂点は0始まりにし、グラフに追加
        u -= 1;
        v -= 1;

        graph[u].push(v);
        graph[v].push(u);
    }

    // 各頂点の色リスト初期値
    let mut colors: Vec<usize> = Vec::new();
    for _i in 0..n {
        proconio::input! {
            c:usize
        }
        colors.push(c);
    }

    // クエリ実行
    for _i in 0..q {
        proconio::input! {
            query_type:usize,
            mut x:usize
        }
        // まず頂点xの色を出力(頂点を0スタートにする)
        x -= 1;
        println!("{}", colors[x]);

        // クエリタイプ1の場合
        // スプリンクラー起動(隣接点をすべてxと同じ色にする)
        if query_type == 1 {
            for &i in &graph[x] {
                colors[i] = colors[x];
            }
        } else {
            // クエリタイプ2の場合
            // 頂点xの色をyで上書き
            proconio::input! {
                mut y:usize
            }
            colors[x] = y;
        }
    }
}
