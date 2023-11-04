use std::collections::VecDeque;

// initの地点からの距離を求める。
// okじゃないとこには移動しない
fn bfs(
    h: usize,
    w: usize,
    init: (usize, usize),
    grid: &Vec<Vec<char>>,
    ok: char,
) -> Vec<Vec<usize>> {
    let mut visited = vec![vec![false; w]; h];
    let mut res = vec![vec![1 << 60; w]; h];
    let mut q = VecDeque::new();

    visited[init.0][init.1] = true;
    res[init.0][init.1] = 0usize;
    q.push_back((init.0, init.1));
    while let Some((x, y)) = q.pop_front() {
        let d = res[x][y];
        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);
            if h <= nx || w <= ny {
                continue;
            }
            if visited[nx][ny] {
                continue;
            }
            if grid[nx][ny] == ok && d < res[nx][ny] {
                q.push_back((nx, ny));
                visited[nx][ny] = true;
                res[nx][ny] = d + 1;
            }
        }
    }
    res
}

fn move_grid_4(n: usize, x: usize, y: usize) {
    for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
        let xi = x.wrapping_add(dx);
        let yi = y.wrapping_add(dy);
        if n <= xi || n <= yi {
            continue;
        }
        // ここは移動可能
    }
}

fn move_grid_8(n: usize, x: usize, y: usize) {
    let dx = vec![!0, !0, 0, 1, 1, 1, 0, !0];
    let dy = vec![0, 1, 1, 1, 0, !0, !0, !0];
    for i in 0..8 {
        let xi = x.wrapping_add(dx[i]);
        let yi = y.wrapping_add(dy[i]);
        if n <= xi || n <= yi {
            continue;
        }
        // ここは移動可能
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

// targetの領域の数をかぞえる
fn count_area(grid: &mut Vec<Vec<bool>>, w: usize, h: usize, target: bool) -> usize {
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    let mut ans = 0;
    for y in 0..w {
        for x in 0..h {
            if grid[y][x] != target {
                // 対象外
                continue;
            }
            ans += 1;
            let mut q = VecDeque::new();
            q.push_back((x, y));
            while let Some((sx, sy)) = q.pop_front() {
                for i in 0..4 {
                    let tx = sx as i64 + dx[i];
                    let ty = sy as i64 + dy[i];
                    if tx < 0 || w as i64 <= tx || ty < 0 || w as i64 <= ty {
                        continue;
                    }
                    let tx = tx as usize;
                    let ty = ty as usize;
                    if grid[ty][tx] != target {
                        continue;
                    }
                    q.push_back((tx, ty));
                    grid[ty][tx] = !target;
                }
            }
        }
    }
    ans
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
    n: usize,
    grid: Vec<Vec<usize>>,
}

impl TwoDSum {
    pub fn new(n: usize) -> Self {
        TwoDSum {
            n,
            grid: vec![vec![0; n + 1]; n + 1],
        }
    }

    pub fn execute(&mut self) {
        for i in 0..=self.n {
            for j in 0..self.n {
                self.grid[i][j + 1] = self.grid[i][j + 1] + self.grid[i][j];
            }
        }
        for i in 0..self.n {
            for j in 0..=self.n {
                self.grid[i + 1][j] = self.grid[i + 1][j] + self.grid[i][j];
            }
        }
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
