use superslice::Ext;

fn simple_bit_search(n: usize) {
    for i in 0..1 << n {
        let mut count = vec![false; n];
        for j in 0..n {
            if i >> j & 1 == 1 {
                // フラグが立っている
                count[j] = true;
            } else {
                // フラグが立っていない
            }
        }
    }
}

// supの中を全探索したい
fn subset_search(sup: usize) {
    let mut sub = sup;
    while sub > 0 {
        println!("{}", sub);
        sub = sub - 1 & sup;
        if sub == sup {
            break;
        }
    }
}

// n以下のサイズkの部分集合
fn size_subset_search(n: usize, k: usize) {
    let mut comb = (1 << k) - 1;
    while comb < 1 << n {
        println!("{}", comb);
        let x = comb & -comb;
        let y = comb + x;
        comb = ((comb & !y) / x >> 1) | y;
    }
}

// 間に合わない場合はlog落とせる
// これは足し合わせてt以下の最大の数をもとめる問題
// https://fairy-lettuce.hatenadiary.com/entry/2020/11/23/084343
fn half_bit_search(n: usize, t: usize, a: Vec<usize>) -> usize {
    let n1 = n / 2;
    let n2 = n - n1;
    // 前半部分の全列挙
    let mut first = vec![];
    for i in 0..1 << n1 {
        let mut sa = 0;
        for j in 0..n1 {
            if i >> j & 1 == 1 {
                sa += a[j];
            }
        }
        if sa <= t {
            first.push(sa);
        }
    }
    // 辞書順にソート
    first.sort();
    let mut ans = 0;
    // 後半部分の全列挙
    for i in 0..1 << n2 {
        let mut sa = 0;
        for j in 0..n2 {
            if i >> j & 1 == 1 {
                sa += a[n1 + j];
            }
        }
        if sa <= t {
            let pos = first.upper_bound(&(t - sa)) - 1;
            ans = ans.max(sa + first[pos]);
        }
    }
    ans
}

// 巡回セールスマン問題
fn bit_dp_template(n: usize, uvc: Vec<(usize, usize, usize)>) -> usize {
    let inf = 1 << 60;
    let mut edges = vec![vec![]; n];
    for (u, v, c) in uvc {
        edges[u].push((v, c));
    }
    // dp[S][i]
    // Sがこれまで通ってきた点の集合(右が0、左がnのbit)
    // iが直前にいたマス
    let mut dp = vec![vec![inf; n]; 1 << n];
    dp[0][0] = 0;
    for i in 0..1 << n {
        for j in 0..n {
            if dp[i][j] == inf {
                // 到達不可
                continue;
            }
            for &(next, cost) in &edges[j] {
                if i >> next & 1 == 1 {
                    // 訪問済み
                    continue;
                }
                dp[i | 1 << next][next] = dp[i | 1 << next][next].min(dp[i][j] + cost);
            }
        }
    }
    dp[(1 << n) - 1][0]
}
