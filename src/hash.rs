// RollingHash + SegmentTree
// ABC331 F

pub struct RollingHash {
    b: Vec<u128>,
    hash: Vec<u128>,
}

impl RollingHash {
    // const BASE:u128 = 1000001137;
    pub fn init(base: u128, n: usize, str: &Vec<char>) -> Self {
        let mut b = vec![1u128];
        let mut hash = vec![0u128];
        let mut t = 1;
        for i in 0..n {
            hash.push((str[i] as u128).wrapping_mul(t).wrapping_add(hash[i]));
            t = t.wrapping_mul(base);
            b.push(t);
        }
        Self { b, hash }
    }

    // i..jまでの文字列のハッシュ値
    // 位置が違うところで比較するときは、b[差分]だけかけてから比較すること
    fn hash(&self, i: usize, j: usize) -> u128 {
        self.hash[j].wrapping_sub(self.hash[i])
    }
}

// strのi..jまでの文字列のハッシュ値を返却する
fn rolling_hash(str: &Vec<char>, i: usize, j: usize) -> u128 {
    let base = 1000001137;
    let mut hash = 0u128;
    let mut t = 1;
    for i in i..j {
        hash = hash.wrapping_add((str[i] as u128).wrapping_mul(t));
        t = t.wrapping_mul(base);
    }
    hash
}

pub struct ModRollingHash {
    modulo: u128,
    b: Vec<u128>,
    hash: Vec<u128>,
}

impl ModRollingHash {
    // const MOD: u128 = 1_000_000_007;
    // const BASE: u128 = 10007;
    pub fn init(modulo: u128, base: u128, n: usize, str: &Vec<char>) -> Self {
        let mut b = vec![1u128];
        let mut hash = vec![0u128];
        let mut t = 1;
        for i in 0..n {
            hash.push(((str[i] as u128 * t % modulo) + hash[i]) % modulo);
            t = (t * base) % modulo;
            b.push(t);
        }
        Self { modulo, b, hash }
    }

    // i..jまでの文字列のハッシュ値
    // 位置が違うところで比較するときは、b[差分]だけかけてから比較すること
    fn hash(&self, i: usize, j: usize) -> u128 {
        (self.modulo + self.hash[j] - self.hash[i]) % self.modulo
    }
}
