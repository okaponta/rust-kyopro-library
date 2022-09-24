use itertools::Itertools;

// 2つのベクトルの内積を返却
// これが負なら、ベクトルのなす角は90度以上
fn inner_product(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    x0 * x1 + y0 * y1
}

// 2つのベクトルの外積を返却
// これの絶対値がAB/ACのつくる平行四辺形の面積
// 正なら時計周り、負なら半時計周り、0なら一直線上
// 外積の絶対値をABの長さで割れば点と線の距離が求まる。
fn outer_product(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    x0 * y1 - y0 * x1
}

// 上記の点だけ渡す版
// P1P2とP1P3の外積を求める。外積なので三角形の二倍
fn outer_product_p(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i64 {
    let a = p2.0 - p1.0;
    let b = p2.1 - p1.1;
    let c = p3.0 - p1.0;
    let d = p3.1 - p1.1;
    a * d - b * c
}

// 距離の二乗を返却
fn dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}

// 3点が同一線状にあるか判定
fn is_on_line(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    (y1 - y0) * (x2 - x0) == (y2 - y0) * (x1 - x0)
}

// 2点を渡すと、中心と半径を返却する
fn circum_from_two(x0: i32, y0: i32, x1: i32, y1: i32) -> (f64, f64, f64) {
    (
        (x0 + x1) as f64 / 2.,
        (y0 + y1) as f64 / 2.,
        ((x0 - x1) as f64).hypot((y0 - y1) as f64),
    )
}

// 3点を渡すと、中心と半径を返却する
fn circum_from_three(x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> (f64, f64, f64) {
    let (x1, y1) = ((x1 - x0) as f64, (y1 - y0) as f64);
    let (x2, y2) = ((x2 - x0) as f64, (y2 - y0) as f64);
    let q = (x1 * x1 * x2 - x2 * x2 * x1 + y1 * y1 * x2 - y2 * y2 * x1) / (y1 * x2 - y2 * x1) / 2.;
    let p = (x1 * x1 + y1 * y1 - 2. * q * y1) / x1 / 2.;
    let r = p.hypot(q) + 1e-8;
    (p + x0 as f64, q + y0 as f64, r)
}

// (x^2 + y^2).sqrt() = x.hypot(y)
// xyのどれかが(x0,y0)を中心の半径rの円の内部にあるかを返却
fn is_inside(x0: f64, y0: f64, r: f64, xy: &Vec<(i32, i32)>) -> bool {
    !xy.iter()
        .any(|&(x, y)| (x as f64 - x0).hypot(y as f64 - y0) > r)
}

// 最近点対問題(最も近い2点を分割統治法で求める)
fn closest(mut points: Vec<(i64, i64)>) -> f64 {
    points.sort_by_key(|p| (p.0));
    let square = closest_pair(&points, 0, points.len()).0;
    (square as f64).sqrt()
}

// 最近点対問題の補助関数。再起的に呼び出す。
// 誤差の考慮をなくすために戻り値は距離の二乗
fn closest_pair(points: &Vec<(i64, i64)>, i: usize, n: usize) -> (i64, Vec<(i64, i64)>) {
    if n <= 1 {
        return (1 << 60, vec![points[i]]);
    }
    let m = n / 2;
    let (d1, qs1) = closest_pair(points, i, m);
    let (d2, qs2) = closest_pair(points, i + m, n - m);
    let mut d = d1.min(d2);

    let x = points[i + m].0;
    let qs = qs1
        .into_iter()
        .merge_by(qs2.into_iter(), |x, y| x.1 < y.1)
        .collect_vec();
    let mut b: Vec<(i64, i64)> = vec![];
    for i in 0..n {
        if (qs[i].0 - x).abs() * (qs[i].0 - x).abs() >= d {
            continue;
        }
        for j in (0..b.len()).rev() {
            let dx = qs[i].0 - b[j].0;
            let dy = qs[i].1 - b[j].1;
            if dy * dy >= d {
                break;
            }
            d = d.min(dx * dx + dy * dy);
        }
        b.push(qs[i]);
    }
    (d, qs)
}

// 凸多角形の面積の2倍を求める
// 三角形に分割して計算を行う
fn convex_area(xy: Vec<(i64, i64)>) -> i64 {
    let mut res = 0;
    let n = xy.len();
    for i in 2..n {
        res += outer_product_p(xy[0], xy[i - 1], xy[i]).abs();
    }
    res
}

// x,yを半時計まわりにd度回転させる
fn rotate(x: f64, y: f64, d: f64) -> (f64, f64) {
    let p = nalgebra::Vector2::new(x, y);
    let rot = nalgebra::Rotation2::new(d * 2.0 * std::f64::consts::PI / 360.0);
    let res = rot * p;
    (res[0], res[1])
}

// アフィン変換(189eを見てね)

// 円周が重なるかどうかを返却する。接する場合もtrueを返却する
fn is_cross_circumference(x1: i64, y1: i64, r1: i64, x2: i64, y2: i64, r2: i64) -> bool {
    let d = dist(x1, y1, x2, y2);
    if (r1 + r2).pow(2) < d {
        return false;
    }
    if d < (r1 - r2).pow(2) {
        return false;
    }
    true
}
