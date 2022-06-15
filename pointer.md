# ライブラリ化しないけど見直したいかも

## いもす
- 尺取り法的な感じで区間に対して足すときに、以下のように管理しておく
  - 開始座標 +x
  - 終了座標 -x
- 参考実装
  - [区間の和と定数のどちらが大きいか検証](https://github.com/okaponta/atcoder-rust/blob/master/abc188/src/bin/d_ex.rs)
    - 最初からわかってるので vec で処理
  - [範囲に対して攻撃するシミュレーション](https://github.com/okaponta/atcoder-rust/blob/master/abc153/src/bin/f.rs)
    - 最初はわからないので vecdeque で処理

## ループを求めて計算
- [ループを求めて累積和を計算](https://github.com/okaponta/atcoder-rust/blob/master/abc241/src/bin/e.rs)

## 高速ゼータ変換
- s & t == tのとき、N×2^N に計算量を圧縮
- 以下どっちでも同じ
  - if (i & 1 << bit) != 0 { a[i] += a[i & !(1 << bit)]; }
  - if (i & 1 << bit) == 0 { a[i | 1 << bit] += a[i]; }
  - [二項係数](https://github.com/okaponta/atcoder-rust/blob/master/arc137/src/bin/d.rs)

## BinaryTreeSet
- 次の要素や最大値などを素早く取得したいとき
- ソートされた状態で管理。クエリとかで挿入しながら取得したりするときに使える
  - [次の要素を高速で取得](https://github.com/okaponta/atcoder-rust/blob/master/abc228/src/bin/d.rs)
  - [index を含めて重複する数字を管理](https://github.com/okaponta/atcoder-rust/blob/master/abc217/src/bin/e.rs)
  - [配列に数字を挿入しながら指定された数字から k 番目を出力](https://github.com/okaponta/atcoder-rust/blob/master/abc241/src/bin/d.rs)
  - [大きさをもった状態で管理して処理したい](https://github.com/okaponta/atcoder-rust/blob/master/abc245/src/bin/e.rs)

## BinaryTreeMap
- ソートされた状態で map に格納する
  - [辞書順に出現回数が最大のものを出力](https://github.com/okaponta/atcoder-rust/blob/master/abc155/src/bin/c_ex.rs)

## Segment tree
- 区間上の値を更新する
- 任意の区間上の最小値や合計値などを取得する
  - [区間の XOR を更新しながら計算](https://github.com/okaponta/atcoder-rust/blob/master/abc185/src/bin/f.rs)
  - [累積和とその最小値を管理](https://github.com/okaponta/atcoder-rust/blob/master/abc223/src/bin/f.rs)

## Mo's Algorithm
- クエリ先読み可能
- 配列が不変
- (l,r)から(l-+1,r-+1)が簡単に計算可能
  - [2で割ったカウント](https://github.com/okaponta/atcoder-rust/blob/master/abc242/src/bin/g.rs)

## Bitdp
- [三次元の巡回セールスマン](https://github.com/okaponta/atcoder-rust/blob/master/abc180/src/bin/e.rs)
- [並び替えの問題、転倒数](https://github.com/okaponta/atcoder-rust/blob/master/abc232/src/bin/f.rs)
    - 転倒数はここらへん見る。順列を並び替えるのに必要なswap回数
      - https://scrapbox.io/pocala-kyopro/%E8%BB%A2%E5%80%92%E6%95%B0

## 桁dp
- [1 の出現する回数](https://github.com/okaponta/atcoder-rust/blob/master/abc029/src/bin/d.rs)
- [長い数で特定の数が出てくる総和](https://github.com/okaponta/atcoder-rust/blob/master/abc235/src/bin/f.rs)

## 包除定理
- 1つはプラス、2つはマイナス、3つはプラス・・・みたいな感じで交互にやってく
- 参考実装
  - [bit演算×2](https://github.com/okaponta/atcoder-rust/blob/master/abc246/src/bin/f.rs)
  - [いずれかの倍数(num_multiple)](src/integer.rs)

## 基底ベクトル
- i = i.min(i ^ e);としてそれを基底としてぶちこむ
  - [貪欲に基底ベクトルを見つける](https://github.com/okaponta/atcoder-rust/blob/master/abc236/src/bin/f.rs)
