use num_integer::Roots;
use superslice::Ext;

// 平方分割テンプレ
fn square_div_template(n: usize, a: Vec<i64>, ijk: Vec<(usize, usize, usize)>) {
    // iからjのうち、k番目に大きいものを求める問題
    let b = n.sqrt();
    let mut buckets = vec![vec![]; n / b + 1];
    let mut nums = vec![];
    for i in 0..n {
        buckets[i / b].push(a[i]);
        nums.push(a[i]);
    }
    nums.sort();
    for i in 0..n / b {
        buckets[i].sort();
    }

    for (i, j, k) in ijk {
        let mut lower = 0;
        let mut upper = n;
        while upper - lower > 1 {
            let med = (lower + upper) / 2;
            let x = nums[med];
            let mut tl = i;
            let mut tr = j;
            let mut count = 0;
            // 左をバケットの境目まで足していく
            while tl < tr && tl % b != 0 {
                if a[tl] <= x {
                    count += 1;
                }
                tl += 1;
            }
            // 右をバケットの境目まで引いていく
            while tl < tr && tr % b != 0 {
                tr -= 1;
                if a[tr] <= x {
                    count += 1;
                }
            }
            // ここまできたら、バケットごとにソートされている
            while tl < tr {
                let target = tl / b;
                count += buckets[target].upper_bound(&x);
                tl += b;
            }

            if count >= k {
                upper = med;
            } else {
                lower = med;
            }
        }
        println!("{}", nums[upper]);
    }
}

fn mo_template(n: usize, c: Vec<usize>, lr: Vec<(usize, usize)>) {
    // この問題は範囲の数の種類を答える問題だった
    let mut query = vec![];
    let blocks = n.sqrt(); // ブロックの数
    for (i, &(l, r)) in lr.iter().enumerate() {
        query.push((i, l, r, l / blocks)); // インデックス/l/r/ブロックの番号
    }
    // ブロック順、同じブロックはr順でソート
    query.sort_by(|a, b| a.3.cmp(&b.3).then(a.2.cmp(&b.2)));

    let mut ans_arr = vec![];

    let mut count = vec![0; n + 1];
    let mut l_tmp = 0;
    let mut r_tmp = 0;
    let mut ans = 0;
    for (i, l, r, _) in query {
        // lが大きければ減少させる(範囲拡張)
        while l_tmp > l {
            l_tmp -= 1;
            if count[c[l_tmp]] == 0 {
                ans += 1;
            }
            count[c[l_tmp]] += 1;
        }
        // rが小さければ増加させる(範囲拡張)
        while r_tmp < r {
            if count[c[r_tmp]] == 0 {
                ans += 1;
            }
            count[c[r_tmp]] += 1;
            r_tmp += 1;
        }
        // lが小さければ増加させる(範囲縮小)
        while l_tmp < l {
            count[c[l_tmp]] -= 1;
            if count[c[l_tmp]] == 0 {
                ans -= 1;
            }
            l_tmp += 1;
        }
        // rが大きければ減少させる(範囲縮小)
        while r_tmp > r {
            r_tmp -= 1;
            count[c[r_tmp]] -= 1;
            if count[c[r_tmp]] == 0 {
                ans -= 1;
            }
        }
        ans_arr.push((i, ans));
    }
    ans_arr.sort();
    for r in ans_arr {
        println!("{}", r.1);
    }
}
