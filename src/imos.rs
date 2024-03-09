pub struct TwoDImos {
    h: usize,
    w: usize,
    s: Vec<Vec<i64>>,
}

impl TwoDImos {
    pub fn new(h: usize, w: usize) -> Self {
        let s = vec![vec![0; w + 1]; h + 1];
        TwoDImos { h, w, s }
    }

    pub fn add(&mut self, min: (usize, usize), max: (usize, usize), num: i64) {
        self.s[min.0][min.1] += num;
        self.s[min.0][max.1] -= num;
        self.s[max.0][min.1] -= num;
        self.s[max.0][max.1] += num;
    }

    pub fn execute(&mut self) {
        for i in 0..=self.h {
            for j in 1..=self.w {
                self.s[i][j] += self.s[i][j - 1];
            }
        }
        for j in 0..=self.w {
            for i in 1..=self.h {
                self.s[i][j] += self.s[i - 1][j];
            }
        }
    }
}
