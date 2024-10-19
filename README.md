# 概要
Rustの勉強と競プロへ同時に入門してみます。

# グラフ
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

# 累積和
- ある区間(l~r)の総和を求める問題では、最初に累積和Sを作っておき、その区間の差`S(r)-S(l)`を求めると良い。
- 長さNの配列があったとき、**累積和は長さN+1**であることに注意(先頭が0のため)

 # 再帰
再帰関数内では、終了条件を関数トップで定義。
そして、終了条件に近づくように、引数を変化させ再帰呼び出しする。
 ```rust
 fn sum(n: usize) -> usize {
    // 終了条件
    if n == 0 {
        return 0;
    }

    // 再帰呼び出し
    return n + sum(n - 1);
}

pub fn main() {
    println!("{}", sum(10));
}
```
