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

fn subset_search(sup: usize) {
    let mut sub = sup;
    loop {
        println!("{}", sub);
        sub = (sub - 1) & sup;
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
