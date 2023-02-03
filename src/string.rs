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
