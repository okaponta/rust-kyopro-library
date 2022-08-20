// 行列式のn乗
fn pow(mut a: Vec<Vec<usize>>, mut n: usize, modulo: usize, size: usize) -> Vec<Vec<usize>> {
    let mut b = vec![vec![0; size]; size];
    for i in 0..size {
        b[i][i] = 1;
    }
    while 0 < n {
        if n & 1 == 1 {
            b = multiply(&b, &a, size);
            rem(&mut b, modulo, size);
        }
        a = multiply(&a, &a, size);
        rem(&mut a, modulo, size);
        n >>= 1;
    }
    b
}

// 行列式の掛け算
fn multiply(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                res[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    res
}

// 各項のmodをとる
fn rem(a: &mut Vec<Vec<usize>>, modulo: usize, n: usize) {
    for i in 0..n {
        for j in 0..n {
            a[i][j] %= modulo;
        }
    }
}
