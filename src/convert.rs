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
