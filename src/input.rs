use std::io::stdin;

fn input_for_graph(n: usize) {
    let uv = vec![(0, 1)];
    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
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

// インタラクティブな場合はこちらを使用、以下のように使用
// let n: usize = read().parse().unwrap();
fn read() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).ok();
    buf.split_whitespace().next().unwrap().to_string()
}
