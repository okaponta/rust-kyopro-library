// 最長共通接頭辞の長さ
// 例えば、hohohehoiなら
// 9,0,2,0,1,0,2,0,0
fn z(s: &Vec<&char>) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n + 1];
    res[0] = n;
    let mut i = 1;
    let mut j = 0;
    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + res[k] < j {
            res[i + k] = res[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}

// sの中にtはあるか？
fn contains(s: &Vec<char>, t: &Vec<char>) -> bool {
    let ts = t.iter().chain(s.iter()).collect::<Vec<_>>();
    let z = z(&ts);
    for i in t.len()..ts.len() {
        if t.len() <= z[i] {
            return true;
        }
    }
    false
}

// 文字列置換
// println!("{}", s.replace("na", "nya"));

// 正規表現
// let re = Regex::new(r"^A[a-z]+C[a-z]+$").unwrap();
// println!("{}", if re.is_match(&s) { "AC" } else { "WA" });

fn convert_to_array(s: &Vec<char>) -> [usize; 26] {
    let mut res = [0; 26];
    for i in 0..s.len() {
        let c = (s[i] as u8 - b'a') as usize;
        res[c] += 1;
    }
    res
}

// aとbが順序通りに並んでるかを判定する
fn is_ordered(a: ([usize; 26], bool), b: ([usize; 26], bool)) -> bool {
    if !a.1 || !b.1 {
        return false;
    }
    let a_edge = calc_edge(a.0);
    let b_edge = calc_edge(b.0);
    a_edge.1 <= b_edge.0
}

// 最初と最後のアルファベットを返却する
fn calc_edge(a: [usize; 26]) -> (usize, usize) {
    let start = (0..26).into_iter().find(|&i| a[i] != 0).unwrap_or(26);
    let end = (0..26).into_iter().rev().find(|&i| a[i] != 0).unwrap_or(0);
    (start, end)
}

// aはbに含まれているか？
// ローリングハッシュを用いて実装
// 未検証
fn contain_hash(a: Vec<char>, b: Vec<char>) -> bool {
    let base = 1000001137;
    let al = a.len();
    let bl = b.len();
    if bl < al {
        return false;
    }

    let mut t = 1u128;
    for _ in 0..al {
        t = t.wrapping_mul(base);
    }

    let mut ah = 0u128;
    let mut bh = 0u128;
    for i in 0..al {
        // ハッシュ値が0になるとすぐ衝突するので、1以上にする
        ah = ah
            .wrapping_mul(base)
            .wrapping_add((a[i] as u8 - b'a' + 1) as u128);
    }
    for i in 0..bl {
        bh = bh
            .wrapping_mul(base)
            .wrapping_add((b[i] as u8 - b'a' + 1) as u128);
    }

    // bの場所を1文字ずつ進めながらハッシュ値をチェック
    for i in 0..=bl - al {
        if ah == bh {
            return true;
        }
        if i + al < bl {
            bh = bh
                .wrapping_mul(base)
                .wrapping_add((b[i + al] as u8 - b'a' + 1) as u128)
                .wrapping_sub((b[i] as u8 - b'a' + 1) as u128 * t);
        }
    }
    false
}

// 接尾辞配列を構成する
fn suffix_array(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut sa = vec![0; n + 1];
    let mut rank = vec![0; n + 1];
    let mut tmp = vec![0; n + 1];

    for i in 0..=n {
        sa[i] = i;
        rank[i] = if i < n { s[i] as i32 } else { -1 };
    }

    fn compare_sa(i: usize, j: usize, k: usize, n: usize, rank: &Vec<i32>) -> std::cmp::Ordering {
        if rank[i] != rank[j] {
            return rank[i].cmp(&rank[j]);
        }
        let ri = if i + k <= n { rank[i + k] } else { -1 };
        let rj = if j + k <= n { rank[j + k] } else { -1 };
        ri.cmp(&rj)
    }

    let mut k = 1;
    while k <= n {
        sa.sort_by(|&a, &b| compare_sa(a, b, k, n, &rank));
        tmp[sa[0]] = 0;
        for i in 1..=n {
            tmp[sa[i]] = tmp[sa[i - 1]]
                + if compare_sa(sa[i - 1], sa[i], k, n, &rank) == std::cmp::Ordering::Less {
                    1
                } else {
                    0
                };
        }
        for i in 0..=n {
            rank[i] = tmp[i];
        }
        k *= 2;
    }
    sa
}

// 高さ配列(設備時配列における隣同士の接尾辞で先頭何文字が共通しているか)
fn lcp(s: &Vec<char>, sa: &Vec<usize>) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0; n + 1];
    for i in 0..=n {
        rank[sa[i]] = i;
    }

    let mut h = 0;
    let mut lcp = vec![0; n + 1];
    for i in 0..n {
        let j = sa[rank[i] - 1];
        if 0 < h {
            h -= 1;
        }
        while j + h < n && i + h < n {
            if s[j + h] != s[i + h] {
                break;
            }
            h += 1;
        }
        lcp[rank[i] - 1] = h;
    }
    lcp
}

// 未検証
// suffix arrayを用いた判定
// sはtに含まれているかを|t|log|s|で求める
fn sa_contains(s: &Vec<char>, t: &Vec<char>, sa: &Vec<usize>) -> bool {
    let mut lower = 0;
    let mut upper = s.len();
    let n = t.len();
    while 1 < upper - lower {
        let mid = (lower + upper) / 2;
        if s[sa[mid]..sa[mid] + n] < t[0..n] {
            lower = mid;
        } else {
            upper = mid;
        }
    }
    s[sa[upper]..sa[upper] + n] == t[0..n]
}

// 禁則文字列系
// see: https://github.com/okaponta/atcoder-rust/blob/master/abc305/src/bin/g.rs
// 注意：matrix[from][to]
// bit表現したときの数字と文字列の長さを返却する
fn to_bit(one: char, target: Vec<char>) -> (usize, usize) {
    let mut res = 0;
    for i in 0..target.len() {
        if target[i] == one {
            res += 1 << (target.len() - 1 - i)
        }
    }
    (res, target.len())
}

// 長さnの禁則文字列を含まない一覧を返却する
fn all(n: usize, s: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut res = vec![];
    let mut one = vec![true, true];
    for &ban in s {
        if ban.1 == 1 {
            one[ban.0] = false;
        }
    }
    for i in 0..2 {
        if one[i] {
            res.push(i);
        }
    }
    for i in 1..n {
        let mut next = vec![];
        for &j in &res {
            let nxa = j << 1;
            if check(nxa, i + 1, &s) {
                next.push(nxa);
            }
            let nxb = (j << 1) + 1;
            if check(nxb, i + 1, &s) {
                next.push(nxb);
            }
        }
        res = next;
    }
    res
}

// 禁則文字列か判定する
// sの第一項はbit表現した値、第二項は長さ
fn check(target: usize, len: usize, s: &Vec<(usize, usize)>) -> bool {
    for ban in s {
        if len < ban.1 {
            continue;
        }
        if target & ((1 << ban.1) - 1) == ban.0 {
            return false;
        }
    }
    true
}
