use std::collections::VecDeque;

// initの地点からの距離を求める。
// ngのとこには移動しない
fn bfs(
    h: usize,
    w: usize,
    init: (usize, usize),
    grid: &Vec<Vec<char>>,
    ng: char,
) -> Vec<Vec<usize>> {
    let mut visited = vec![vec![false; w]; h];
    let mut res = vec![vec![1 << 60; w]; h];
    let mut q = VecDeque::new();

    visited[init.0][init.1] = true;
    res[init.0][init.1] = 0usize;
    q.push_back((init.0, init.1));
    while let Some((i, j)) = q.pop_front() {
        let d = res[i][j];
        for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if h <= ni || w <= nj {
                continue;
            }
            if visited[ni][nj] || grid[ni][nj] == ng {
                continue;
            }
            q.push_back((ni, nj));
            visited[ni][nj] = true;
            res[ni][nj] = d + 1;
        }
    }
    res
}

fn move_grid_4(h: usize, w: usize, i: usize, j: usize) {
    for (di, dj) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
        let ni = i.wrapping_add(di);
        let nj = j.wrapping_add(dj);
        if h <= ni || w <= nj {
            continue;
        }
        // ここは移動可能
    }
}

fn move_grid_8(n: usize, x: usize, y: usize) {
    let dx = vec![!0, !0, 0, 1, 1, 1, 0, !0];
    let dy = vec![0, 1, 1, 1, 0, !0, !0, !0];
    for i in 0..8 {
        let nx = x.wrapping_add(dx[i]);
        let ny = y.wrapping_add(dy[i]);
        if n <= nx || n <= ny {
            continue;
        }
        // ここは移動可能
    }
}

// let ni = tmp.0.wrapping_add(di);
// let nj = tmp.1.wrapping_add(dj);
// if h <= ni || w <= nj || s[ni][nj] == '#' {
//     continue 'a;
// }
fn dir_to_int(c: char) -> (usize, usize) {
    match c {
        'U' => (!0, 0),
        'D' => (1, 0),
        'L' => (0, !0),
        'R' => (0, 1),
        _ => panic!(),
    }
}

// 連結であるかをチェックする
// c..チェック対象の数字
// n..盤面の大きさ
fn is_renketsu(b: &Vec<Vec<usize>>, c: usize, n: usize) -> bool {
    let mut q = VecDeque::new();
    for i in 0..(n * n) {
        if b[i / n][i % n] == 1 {
            q.push_back((i / n, i % n));
            break;
        }
    }
    let mut visited = vec![vec![false; n]; n];
    while let Some((x, y)) = q.pop_front() {
        visited[x][y] = true;

        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let xi = x.wrapping_add(dx);
            let yi = y.wrapping_add(dy);
            if n <= xi || n <= yi {
                continue;
            }
            if b[xi][yi] == c && !visited[xi][yi] {
                q.push_back((xi, yi));
            }
        }
    }

    (0..n).all(|i| (0..n).all(|j| b[i][j] != c || visited[i][j]))
}

// 穴があるかをチェックする(外から連結であるかをチェックします)
// c..チェック対象の数字(cじゃないものを判定します)
// n..盤面の大きさ
fn has_no_hole(b: &Vec<Vec<usize>>, c: usize, n: usize) -> bool {
    let mut q = VecDeque::new();
    for i in 0..n {
        if b[i][0] == 0 {
            q.push_back((i, 0));
        }
        if b[i][n - 1] == 0 {
            q.push_back((i, n - 1));
        }
        if b[0][i] == 0 {
            q.push_back((0, i));
        }
        if b[n - 1][i] == 0 {
            q.push_back((n - 1, i));
        }
    }
    let mut visited = vec![vec![false; n]; n];
    while let Some((x, y)) = q.pop_front() {
        visited[x][y] = true;

        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let xi = x.wrapping_add(dx);
            let yi = y.wrapping_add(dy);
            if n <= xi || n <= yi {
                continue;
            }
            if b[xi][yi] == c && !visited[xi][yi] {
                q.push_back((xi, yi));
            }
        }
    }

    (0..n).all(|i| (0..n).all(|j| b[i][j] != c || visited[i][j]))
}

// targetのcharで分類する(1-indexed。対象意外は0)
fn count_area(grid: &Vec<Vec<char>>, h: usize, w: usize, target: char) -> (usize, Vec<Vec<usize>>) {
    let mut res = vec![vec![0; w]; h];
    let mut q = VecDeque::new();
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != target || res[i][j] != 0 {
                continue;
            }
            count += 1;
            q.push_back((i, j, count));
            res[i][j] = count;
            while let Some((x, y, count)) = q.pop_front() {
                for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
                    let nx = x.wrapping_add(dx);
                    let ny = y.wrapping_add(dy);
                    if h <= nx || w <= ny {
                        continue;
                    }
                    if grid[nx][ny] == target && res[nx][ny] == 0 {
                        q.push_back((nx, ny, count));
                        res[nx][ny] = count;
                    }
                }
            }
        }
    }
    (count, res)
}

// 空白の行と列を周りからtrimする
// 途中に空行があってもtrimしない
fn trim_empty_rounds(
    mut h: usize,
    mut w: usize,
    mut a: Vec<Vec<char>>,
) -> (usize, usize, Vec<Vec<char>>) {
    while (0..w).into_iter().all(|j| a[0][j] == '.') {
        a.remove(0);
        h -= 1;
    }
    while (0..w).into_iter().rev().all(|j| a[h - 1][j] == '.') {
        a.remove(h - 1);
        h -= 1;
    }
    while (0..h).into_iter().all(|i| a[i][0] == '.') {
        for j in 0..h {
            a[j].remove(0);
        }
        w -= 1;
    }
    while (0..h).into_iter().rev().all(|i| a[i][w - 1] == '.') {
        for j in 0..h {
            a[j].remove(w - 1);
        }
        w -= 1;
    }
    (h, w, a)
}

// 90度回転
fn rotate_2d_vector(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_v = vec![vec!['.'; v.len()]; v[0].len()];
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            new_v[j][v.len() - 1 - i] = v[i][j];
        }
    }
    return new_v;
}

// 左上につめる
fn upleft(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = v.len();
    let h = v[0].len();
    let mut new_v = vec![vec!['.'; w]; h];
    let mut upshift = 0;
    let mut leftshift = 0;
    for i in 0..h {
        if (0..w).into_iter().all(|j| v[i][j] == '.') {
            upshift += 1;
        } else {
            break;
        }
    }
    for j in 0..w {
        if (0..h).into_iter().all(|i| v[i][j] == '.') {
            leftshift += 1;
        } else {
            break;
        }
    }
    for i in upshift..h {
        for j in leftshift..w {
            new_v[i - upshift][j - leftshift] = v[i][j];
        }
    }
    return new_v;
}

// 二次元累積和
// TODO: きちんと書く
pub struct TwoDSum {
    s: Vec<Vec<usize>>,
}

impl TwoDSum {
    pub fn new(h: usize, w: usize, a: &Vec<Vec<usize>>) -> Self {
        let mut s = vec![vec![0; w + 1]; h + 1];
        for i in 1..=h {
            for j in 1..=w {
                s[i][j] = a[i - 1][j - 1] + s[i - 1][j] + s[i][j - 1] - s[i - 1][j - 1];
            }
        }
        TwoDSum { s }
    }

    // [h1,h2) * [w1,w2)の区間和を返却する
    pub fn get(&mut self, h1: usize, w1: usize, h2: usize, w2: usize) -> usize {
        self.s[h2][w2] + self.s[h1][w1] - self.s[h1][w2] - self.s[h2][w1]
    }

    // [0,h) * [0,w0)の区間和を返却する
    pub fn get_o(&mut self, h: usize, w: usize) -> usize {
        self.s[h][w] + self.s[0][0] - self.s[0][w] - self.s[h][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_renketsu_ok() {
        let n = 4;
        let b = vec![
            vec![0, 1, 1, 1],
            vec![0, 0, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(is_renketsu(&b, 0, n), true);
        assert_eq!(is_renketsu(&b, 1, n), true);
    }

    #[test]
    fn test_is_renketsu_ng() {
        let n = 5;
        let b = vec![
            vec![0, 1, 1, 1, 0],
            vec![0, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(is_renketsu(&b, 0, n), false);
        assert_eq!(is_renketsu(&b, 1, n), true);
    }

    #[test]
    fn test_has_no_hole_ok() {
        let n = 4;
        let b = vec![
            vec![0, 1, 1, 1],
            vec![0, 0, 1, 1],
            vec![1, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(has_no_hole(&b, 0, n), true);
        assert_eq!(has_no_hole(&b, 1, n), true);
    }

    #[test]
    fn test_has_no_hole_ng() {
        let n = 5;
        let b = vec![
            vec![0, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 0],
            vec![1, 0, 0, 1, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
        ];
        assert_eq!(has_no_hole(&b, 0, n), false);
        assert_eq!(has_no_hole(&b, 1, n), true);
    }
}
