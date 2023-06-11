use super::modint::ModInt;

// 行列式のn乗
fn pow(mut a: Vec<Vec<ModInt>>, mut n: usize) -> Vec<Vec<ModInt>> {
    let mut b = vec![vec![ModInt::zero(); a.len()]; a.len()];
    for i in 0..a.len() {
        b[i][i] = ModInt::one();
    }
    while 0 < n {
        if n & 1 == 1 {
            b = mul(&a, &b);
        }
        a = mul(&a, &a);
        n >>= 1;
    }
    b
}

fn mul(a: &Vec<Vec<ModInt>>, b: &Vec<Vec<ModInt>>) -> Vec<Vec<ModInt>> {
    let n = a.len();
    let mut res = vec![vec![ModInt::zero(); n]; n];
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

// 連立一次方程式の解を求める。
// 求まらない場合は空の配列を返却する。
fn gauss_jordan(n: usize, mat: &Vec<Vec<i64>>, sol: &Vec<i64>) -> Vec<f64> {
    let eps = 1e-8;
    let mut b = vec![vec![0.0; n + 1]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = mat[i][j] as f64;
        }
        b[i][n] = sol[i] as f64;
    }

    for i in 0..n {
        let mut pivot = i;
        for j in i..n {
            if b[j][i].abs() > b[pivot][i].abs() {
                pivot = j;
            }
        }
        (b[i], b[pivot]) = (b[pivot].clone(), b[i].clone());

        if b[i][i].abs() < eps {
            return vec![];
        }

        for j in i + 1..=n {
            b[i][j] /= b[i][i];
        }
        for j in 0..n {
            if i != j {
                for k in i + 1..=n {
                    b[j][k] -= b[j][i] * b[i][k];
                }
            }
        }
    }
    let mut res = vec![];
    for i in 0..n {
        res.push(b[i][n]);
    }
    res
}
