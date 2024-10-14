# 概要
Rustの勉強と競プロへ同時に入門してみます。

# グラフ問題
- 頂点を番号にする。
- 2次元配列に、グラフ構造を格納する(隣接グラフ)。
  - 配列のインデックス：頂点の番号
  - 配列内の配列の要素：隣接する頂点番号
```rust
    // 頂点の数、辺の数、クエリの数
    proconio::input! {
        n:usize,
        m:usize
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
```
