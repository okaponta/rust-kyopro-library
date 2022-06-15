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
