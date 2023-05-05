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
fn contain(a: Vec<char>, b: Vec<char>) -> bool {
    let modulo = 4294967029;

    let base = 3415016898;
    let al = a.len();
    let bl = b.len();
    if bl < al {
        return false;
    }

    let mut t = 1;
    for _ in 0..al {
        t *= base;
        t %= modulo;
    }

    let mut ah = 0;
    let mut bh = 0;
    for i in 0..al {
        // ハッシュ値が0になるとすぐ衝突するので、1以上にする
        ah = (ah * base + (a[i] as u8 - b'a' + 1) as usize) % modulo;
    }
    for i in 0..bl {
        bh = (bh * base + (b[i] as u8 - b'a' + 1) as usize) % modulo;
    }

    // bの場所を1文字ずつ進めながらハッシュ値をチェック
    for i in 0..=bl - al {
        if ah == bh {
            return true;
        }
        if i + al < bl {
            bh = bh * base + (b[i + al] as u8 - b'a' + 1) as usize
                - (b[i] as u8 - b'a' + 1) as usize * t;
        }
    }
    false
}
