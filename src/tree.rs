use itertools::Itertools;
use superslice::Ext;

// segment tree
// seg[0] -> seg[1]+seg[2]
// seg[1] -> seg[3]+seg[4] seg[2] -> seg[5]+seg[6]
// seg[3] -> seg[7]+seg[8] seg[6] -> seg[13]+seg[14]]
// 必要な要素数は2^n-1
// 区間上の値を更新する
// 任意の区間上の最小値や合計値(与えるfuncによって全てのbit or値)などを取得する
pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Copy, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let m = size.next_power_of_two();
        SegmentTree {
            seg: vec![initial_value; m * 2],
            n: m,
            f,
            initial_value,
        }
    }

    pub fn update(&mut self, mut k: usize, value: T) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    pub fn update_tmp(&mut self, k: usize, value: T) {
        self.seg[k + self.n - 1] = value;
    }

    pub fn update_all(&mut self) {
        for i in (0..self.n - 1).rev() {
            self.seg[i] = (self.f)(self.seg[2 * i + 1], self.seg[2 * i + 2]);
        }
    }

    // 半開区間なので注意
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        self.query_range(range, 0, 0..self.n)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
    ) -> T {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            self.initial_value
        } else if range.start <= seg_range.start && seg_range.end <= range.end {
            self.seg[k]
        } else {
            let mid = (seg_range.start + seg_range.end) >> 1;
            let x = self.query_range(range.clone(), k * 2 + 1, seg_range.start..mid);
            let y = self.query_range(range, k * 2 + 2, mid..seg_range.end);
            (self.f)(x, y)
        }
    }
}

// フェニック木。以下2つができる。1-indexedなので注意
// 1. ai に v を加算する
// 2. a1+a2+...+aiを求める
struct FenwickTree {
    len: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    // a1~anの配列を作成
    fn new(n: usize) -> Self {
        Self {
            len: n + 1,
            data: vec![0; n + 1],
        }
    }

    // aiにvを加算する
    fn add(&mut self, i: usize, v: i64) {
        assert!(i > 0);
        assert!(i < self.len);
        let mut i = i as i64;
        while (i as usize) < self.len {
            self.data[i as usize] += v;
            i += i & -i;
        }
    }

    // a1+a2+...aiを計算する
    fn sum(&self, i: usize) -> i64 {
        assert!(i < self.len);
        let mut i = i as i64;
        let mut sum = 0;
        while i > 0 {
            sum += self.data[i as usize];
            i -= i & -i;
        }
        sum
    }

    // ai+...+ajを計算する
    fn range(&self, i: usize, j: usize) -> i64 {
        assert!(i <= j);
        assert!(j < self.len);
        self.sum(j) - self.sum(i - 1)
    }
}

// 転倒数を求める。
// とりうる最大値の木作るので、制約が大きい場合は座標圧縮などしてから呼び出すこと
fn inversion_num(a: Vec<usize>) -> i64 {
    let max = *a.iter().max().unwrap();
    let mut inv = 0;
    let mut fw = FenwickTree::new(max);
    // 注意：1-indexedで行うこと！
    for i in 0..a.len() {
        inv += i as i64 - fw.sum(a[i]);
        fw.add(a[i], 1);
    }
    inv
}

struct LazySegmentTree {
    n: usize,
    node: Vec<i64>,
    lazy: Vec<i64>,
}

impl LazySegmentTree {
    fn new(size: usize) -> LazySegmentTree {
        let n = size.next_power_of_two();
        LazySegmentTree {
            n: n,
            node: vec![0i64; 2 * n],
            lazy: vec![0i64; 2 * n],
        }
    }

    // k番目のノードの遅延評価
    fn eval(&mut self, k: usize, l: usize, r: usize) {
        if self.lazy[k] != 0 {
            self.node[k] += self.lazy[k];
        }
        if r - l > 1 {
            // 最下段かどうかのチェック
            self.lazy[2 * k + 1] += self.lazy[k] / 2;
            self.lazy[2 * k + 2] += self.lazy[k] / 2;
        }
        self.lazy[k] = 0;
    }

    // [a,b)にxを加算する
    fn add(&mut self, a: usize, b: usize, x: i64) {
        self.add_range(a, b, x, 0, 0, self.n)
    }

    // [a,b)の合計値
    fn sum(&mut self, a: usize, b: usize) -> i64 {
        self.sum_range(a, b, 0, 0, self.n)
    }

    fn add_range(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.lazy[k] += (r - l) as i64 * x;
            self.eval(k, l, r);
        } else {
            self.add_range(a, b, x, k * 2 + 1, l, (l + r) / 2);
            self.add_range(a, b, x, k * 2 + 2, (l + r) / 2, r);
            self.node[k] = self.node[2 * k + 1] + self.node[2 * k + 2];
        }
    }

    fn sum_range(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if b <= l || r <= a {
            return 0;
        }
        self.eval(k, l, r);
        if a <= l && r <= b {
            return self.node[k] as i64;
        }
        let left = self.sum_range(a, b, 2 * k + 1, l, (l + r) / 2);
        let right = self.sum_range(a, b, 2 * k + 2, (l + r) / 2, r);
        return left + right;
    }
}

// https://zenn.dev/nakamurus/articles/f398b7f4d7618ea5b7eb
// ↑から拝借。。。あとで書き直す。
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

// 領域セグ木
// 各ノードがソート済の配列をもっている
// この実装だと指定の範囲のとある数字以下の数を数えることができる
struct RangeSegTree {
    data: Vec<Vec<i64>>,
}

impl RangeSegTree {
    fn new(size: usize) -> RangeSegTree {
        let m = size.next_power_of_two();
        let data = vec![vec![]; m * 2];
        RangeSegTree { data }
    }

    fn init(&mut self, a: &Vec<i64>, k: usize, l: usize, r: usize) {
        if r - l == 1 {
            self.data[k].push(a[l]);
        } else {
            let lch = k * 2 + 1;
            let rch = k * 2 + 2;
            let med = (l + r) / 2;
            self.init(a, lch, l, med);
            self.init(a, rch, med, r);
            self.data[k] = self.data[lch]
                .iter()
                .merge(self.data[rch].iter())
                .map(|i| *i)
                .collect::<Vec<i64>>();
        }
    }

    fn query(&self, i: usize, j: usize, x: i64, k: usize, l: usize, r: usize) -> usize {
        if j <= l || r <= i {
            0
        } else if i <= l && r <= j {
            self.data[k].upper_bound(&x)
        } else {
            let med = (l + r) / 2;
            let lc = self.query(i, j, x, k * 2 + 1, l, med);
            let rc = self.query(i, j, x, k * 2 + 2, med, r);
            lc + rc
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment() {
        let mut seg = SegmentTree::new(9, 0, |a, b| a + b);
        println!("{:?}", seg.seg);
        seg.update(1, 1);
        println!("{:?}", seg.seg);
        assert_eq!(seg.query(0..1), 0);
        assert_eq!(seg.query(0..2), 1);

        seg.update(5, 2);
        println!("{:?}", seg.seg);
        assert_eq!(seg.query(0..5), 1);
        assert_eq!(seg.query(0..6), 3);
        assert_eq!(seg.query(5..6), 2);

        seg.update(9, 1);
        println!("{:?}", seg.seg);
        assert_eq!(seg.query(0..9), 3);
        assert_eq!(seg.query(0..10), 4);
        assert_eq!(seg.query(9..10), 1);

        seg.update(2, -3);
        println!("{:?}", seg.seg);
        assert_eq!(seg.query(0..2), 1);
        assert_eq!(seg.query(0..3), -2);
        assert_eq!(seg.query(0..10), 1);
    }

    #[test]
    fn test_fenwick() {
        let mut fw = FenwickTree::new(9);
        fw.add(1, 1);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(0), 0);
        assert_eq!(fw.sum(1), 1);
        assert_eq!(fw.sum(2), 1);
        fw.add(5, 2);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(4), 1);
        assert_eq!(fw.sum(5), 3);
        fw.add(9, 1);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(8), 3);
        assert_eq!(fw.sum(9), 4);
        fw.add(2, -3);
        println!("{:?}", fw.data);
        assert_eq!(fw.sum(1), 1);
        assert_eq!(fw.sum(2), -2);
        assert_eq!(fw.sum(9), 1);
    }

    #[test]
    fn test_inversion_num() {
        let a = vec![3, 1, 5, 4, 2, 9, 6, 8, 7];
        assert_eq!(inversion_num(a), 9);
    }

    #[test]
    fn test_lazy_segment() {
        let mut seg = LazySegmentTree::new(10);
        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // 初期化
        for i in 0..10 {
            seg.add(i, i + 1, a[i]);
        }
        println!("{:?}", seg.node);
        // 0-indexedで3なので、4が想定解
        assert_eq!(seg.sum(3, 4), 4);
        // 全部の和は55
        assert_eq!(seg.sum(0, 10), 55);

        // 2,3,4,5に3を足した
        seg.add(2, 6, 3);
        println!("{:?}", seg.node);
        // 1,2,6,7,8,9,7,8,9,10
        assert_eq!(seg.sum(2, 3), 6);
        assert_eq!(seg.sum(1, 3), 8);
        assert_eq!(seg.sum(5, 7), 16);
    }
}
