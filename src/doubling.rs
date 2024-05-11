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
// init 始点の数
// k    k番目の数字が実質どれかを求める
// next 次の要素がどれかを計算
fn index_with_loop(init: usize, k: usize) -> usize {
    fn next(mut n: usize) -> usize {
        let mut res = n;
        for i in vec![10000, 1000, 100, 10, 1] {
            res += n / i;
            n %= i;
        }
        res % 100000
    }

    let mut v = vec![];
    let mut set = std::collections::HashSet::new();
    let mut tmp = init;
    while !set.contains(&tmp) {
        v.push(tmp);
        set.insert(tmp);
        tmp = next(tmp);
    }
    if k < v.len() {
        return v[k];
    }
    let offset = v.iter().position(|&x| x == tmp).unwrap();
    let l = v.len() - offset;
    v[offset + (k - offset) % l]
}
