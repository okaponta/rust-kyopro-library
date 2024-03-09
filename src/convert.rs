use std::collections::{HashMap, HashSet};

use itertools::Itertools;

// 座標圧縮
// Mapを返却する。戻り値のmap[元の値]が圧縮された値になる
fn compress(source: &[usize]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i);
    }
    result
}

// ソートはあらかじめしてから呼び出すこと
fn run_length_encode<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}

// 累積和
fn ruiseki(a: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}

// 逆順にソートされていること
// kを挿入するとしたらどのインデックスに入るかが返却される(小さい側)。
// [10,9,7,7,3]
// 11 -> 0
// 10 -> 0
// 9 -> 1
// 8 -> 2
// 7 -> 2
// 6 -> 4
fn lower_bound_rev(a: &Vec<usize>, k: usize) -> usize {
    if a[0] <= k {
        return 0;
    }
    let mut lower = 0;
    let mut upper = a.len();
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if a[med] <= k {
            upper = med;
        } else {
            lower = med;
        }
    }
    upper
}

// 逆順にソートされていること
// kを挿入するとしたらどのインデックスに入るかが返却される(大きい側)。
// [10,9,7,7,3]
// 11 -> 0
// 10 -> 1
// 9 -> 2
// 8 -> 2
// 7 -> 4
// 6 -> 4
fn upper_bound_rev(a: &Vec<usize>, k: usize) -> usize {
    if a[0] < k {
        return 0;
    }
    let mut lower = 0;
    let mut upper = a.len();
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if k <= a[med] {
            lower = med;
        } else {
            upper = med;
        }
    }
    upper
}

// 大きいStringをmodとる
// https://atcoder.jp/contests/abc339/tasks/abc339_f
fn mod_bigint(s: &str, m: u64) -> u64 {
    s.chars()
        .fold(0, |x, c| (x * 10 + c.to_digit(10).unwrap() as u64) % m)
}
