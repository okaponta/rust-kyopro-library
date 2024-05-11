const INF: i64 = std::i64::MAX / 2;
const BAN: usize = 200000 + 2;

#[derive(Clone, Copy)]
struct Top2([(i64, usize); 2]);

impl Top2 {
    fn new() -> Self {
        Self([(-INF, BAN), (-INF, BAN + 1)])
    }
    // ここらへんは問題ごとに書き換える
    // https://atcoder.jp/contests/abc345/tasks/abc345_e
    // cが同じ場合はその最大値
    // そうじゃない場合はシンプルに最大値
    fn update(&mut self, v: i64, c: usize) {
        let p = &mut self.0;
        if let Some(x) = p.iter().position(|p| p.1 == c) {
            p[x].0 = p[x].0.max(v);
            if p[0].0 < p[1].0 {
                p.swap(0, 1);
            }
        } else {
            for i in 0..2 {
                if v >= p[i].0 {
                    p[i..].rotate_right(1);
                    p[i] = (v, c);
                    break;
                }
            }
        }
    }
    fn find(&self, c: usize) -> i64 {
        self.0.iter().find(|p| p.1 != c).unwrap().0
    }
}

impl std::fmt::Debug for Top2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (c, v) in self.0 {
            write!(f, "({}, {})", c, v)?;
        }
        Ok(())
    }
}
