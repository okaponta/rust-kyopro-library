use proconio::input;

fn input_for_graph(n: usize) {
    let uv = vec![(0, 1)];
    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }
    let ab = vec![(0, 1)];
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
}

fn input_for_graph_cost(n: usize) {
    let uvw = vec![(0, 1, 2)];
    let mut edges = vec![vec![]; n];
    for (u, v, w) in uvw {
        edges[u].push((v, w));
        edges[v].push((u, w));
    }
}

#[allow(unused_variables)]
fn input_string_as_int() {
    let s: Vec<char> = vec!['1', '2', '4'];
    for c in s {
        let ci = c.to_digit(10).unwrap() as i32;
    }
}

// なんか意地悪なグラフの入力
fn input_sankaku(n: usize) {
    let mut a = vec![vec![0; n]; n];
    for i in 0..n - 1 {
        input! {ai:[usize;n-i-1]}
        for j in 0..n - i - 1 {
            a[i][j + i + 1] = ai[j];
            a[j + i + 1][i] = ai[j];
        }
    }
}

// インタラクティブな場合はこちらを使用
// input_interactive! {
//     n:usize,
// }

// 以下でもOK
// let n: usize = read().parse().unwrap();
fn read() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.split_whitespace().next().unwrap().to_string()
}

// 以下でも使える
// use std::io::{stdin, BufReader};
// use proconio::{input, source::line::LineSource};
// let stdin = stdin();
// let mut source = LineSource::new(BufReader::new(stdin.lock()));

// input! {
//     from &mut source,
//     n: u32
// }
