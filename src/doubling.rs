// https://atcoder.jp/contests/abc167/tasks/abc167_d
fn doubling_template(mut k: usize, mut a: Vec<usize>) {
    let mut ans = 0;
    // 普通にkを変える方法じゃなくて、桁数でぐるぐるまわすでもOK
    while k > 0 {
        if k & 1 > 0 {
            ans = a[ans];
        }
        a = a.iter().map(|&i| a[i]).collect();
        k >>= 1;
    }
    println!("{}", ans + 1);
}

fn fibonacci(n: usize, modulo: usize) -> usize {
    // フィボナッチの漸化式を行列で表現
    let a = nalgebra::Matrix2::new(1, 1, 1, 0);
    let a = pow(a, n, modulo);
    *a.index((1, 0))
}

// 行列式のn乗
fn pow(mut a: nalgebra::Matrix2<usize>, mut n: usize, modulo: usize) -> nalgebra::Matrix2<usize> {
    let mut b = a.clone();
    while 0 < n {
        if n & 1 == 1 {
            b = b * a;
            rem(&mut b, modulo);
        }
        a = a * a;
        rem(&mut a, modulo);
        n >>= 1;
    }
    b
}

// 各項のmodをとる
fn rem(a: &mut nalgebra::Matrix2<usize>, modulo: usize) {
    a.m11 %= modulo;
    a.m12 %= modulo;
    a.m21 %= modulo;
    a.m22 %= modulo;
}

// ダブリングじゃないけど、ループがある問題
// init 始点の数(多くの問題の場合0かと)
// n    要素数
// k    k番目の数字が実質どれかを求める
// map  次の要素がどれかを保持
fn solve_loop(init: usize, n: usize, mut k: usize, map: Vec<usize>) -> usize {
    let mut used = vec![false; n];
    let mut path = vec![];
    let mut next = init;
    loop {
        if used[next] {
            break;
        }
        path.push(next);
        used[next] = true;
        next = map[next];
    }
    // ループのはじまり
    let mut first = 0;
    for i in 0..n {
        if path[i] == next {
            first = i;
            break;
        }
    }
    let loop_size = path.len() - first;
    if k <= first {
        return map[path[k]];
    } else {
        k -= first;
        k %= loop_size;
        return map[path[k + first]];
    }
}
