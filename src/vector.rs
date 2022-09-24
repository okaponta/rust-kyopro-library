#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V2<T> {
    x: T,
    y: T,
}

impl<T> V2<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    pub fn new(x: T, y: T) -> V2<T> {
        V2 { x, y }
    }

    pub fn new_p(p: (T, T)) -> V2<T> {
        V2 { x: p.0, y: p.1 }
    }

    pub fn mul(&mut self, ope: T) -> Self {
        Self {
            x: self.x * ope,
            y: self.y * ope,
        }
    }

    pub fn dot(&self, p: V2<T>) -> T {
        self.x * p.x + self.y * p.y
    }

    pub fn det(&self, p: V2<T>) -> T {
        self.x * p.y - self.y * p.x
    }
}

impl<T> std::ops::Add for V2<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> std::ops::Sub for V2<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

const EPS: f64 = 1e-10;

// 直線p1-p2と直線q1-q2の交点
fn intersection(p1: V2<f64>, p2: V2<f64>, q1: V2<f64>, q2: V2<f64>) -> V2<f64> {
    p1 + (p2 - p1).mul((q2 - q1).det(q1 - p1) / (q2 - q1).det(p2 - p1))
}

// 線分p1-p2上に点qがあるかを判定
fn on_seg(p1: V2<f64>, p2: V2<f64>, q: V2<f64>) -> bool {
    (p1 - q).det(p2 - q).abs() < EPS && (p1 - q).dot(p2 - q) < EPS
}

// 凸包を求める
fn convex_hull(mut ps: Vec<V2<i64>>, n: usize) -> Vec<V2<i64>> {
    ps.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));
    let mut k = 0;
    let mut qs: Vec<V2<i64>> = vec![];
    // 下側凸包の作成
    for i in 0..n {
        while k > 1 && (qs[k - 1] - qs[k - 2]).det(ps[i] - qs[k - 1]) <= 0 {
            k -= 1;
        }
        if k >= qs.len() {
            qs.push(ps[i]);
        } else {
            qs[k] = ps[i];
        }
        k += 1;
    }
    // 上側凸包の作成
    let t = k;
    for i in (0..n - 1).rev() {
        while k > t && (qs[k - 1] - qs[k - 2]).det(ps[i] - qs[k - 1]) <= 0 {
            k -= 1;
        }
        if k == qs.len() {
            qs.push(ps[i]);
        } else {
            qs[k] = ps[i];
        }
        k += 1;
    }
    qs
}
