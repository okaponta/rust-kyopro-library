mod fft {
    use num::Complex;

    // fft::convolve()とかで呼び出してくれ
    pub fn convolve(mut a: Vec<f64>, mut b: Vec<f64>) -> Vec<f64> {
        let s = a.len() + b.len() - 1;
        let t = s.next_power_of_two();
        a.resize(t, 0.0);
        b.resize(t, 0.0);
        let mut fa = fft_real(a);
        let fb = fft_real(b);
        for i in 0..t {
            fa[i] *= fb[i];
        }
        fa = inverse_fft(fa);
        (0..s).into_iter().map(|i| fa[i].re).collect::<Vec<_>>()
    }

    fn fft_real(a: Vec<f64>) -> Vec<Complex<f64>> {
        fft(a
            .into_iter()
            .map(|f| Complex::new(f, 0.0))
            .collect::<Vec<_>>())
    }

    pub fn fft(mut a: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let n = a.len();
        assert!(
            n.count_ones() == 1,
            "the length of array should be power of two"
        );
        let bit = n.trailing_zeros() as usize;

        for si in (0..bit).rev() {
            let s = 1_usize << si;
            let zeta = Complex::from_polar(1.0, 2.0 * std::f64::consts::PI / (s << 1) as f64);
            for ii in 0..(n / (s << 1)) {
                let i = ii * (s << 1);
                let mut z_i = Complex::new(1.0, 0.0);
                for j in 0..s {
                    let t = a[i + j] - a[s + i + j];
                    a[i + j] = a[i + j] + a[s + i + j];
                    a[s + i + j] = t * z_i;
                    z_i *= zeta;
                }
            }
        }

        a
    }

    pub fn inverse_fft(mut a: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
        let n = a.len();
        assert!(
            n.count_ones() == 1,
            "the length of array should be power of two"
        );
        let bit = n.trailing_zeros() as usize;

        for si in 0..bit {
            let s = 1_usize << si;
            let zeta = Complex::from_polar(1.0, -2.0 * std::f64::consts::PI / (s << 1) as f64);
            for ii in 0..(n / (s << 1)) {
                let i = ii * (s << 1);
                let mut z_i = Complex::new(1.0, 0.0);
                for j in 0..s {
                    let t = a[s + i + j] * z_i;
                    a[s + i + j] = a[i + j] - t;
                    a[i + j] = a[i + j] + t;
                    z_i *= zeta;
                }
            }
        }

        let inv_n = Complex::new(1_f64 / n as f64, 0f64);
        a.iter().map(|&x| x * inv_n).collect()
    }
}

mod ntt {
    use super::super::modint::ModInt;

    fn convolve(a: &Vec<usize>, b: &Vec<usize>) -> Vec<ModInt> {
        let s = a.len() + b.len() - 1;
        let t = s.next_power_of_two();

        let root = {
            let z_t_n = ModInt::new(3).pow(119).pow((1 << 23) / t);
            (0..t).map(|i| z_t_n.pow(i)).collect::<Vec<_>>()
        };

        let root_inv = {
            let mut root_inv = root.clone();
            root_inv[1..].reverse();
            root_inv
        };

        let mut a = a.iter().copied().map(ModInt::new).collect::<Vec<_>>();
        let mut b = b.iter().copied().map(ModInt::new).collect::<Vec<_>>();
        a.resize(t, ModInt::new(0));
        b.resize(t, ModInt::new(0));
        let a_inv = ntt(&a, &root);
        let b_inv = ntt(&b, &root);

        let c_inv = a_inv
            .into_iter()
            .zip(b_inv.into_iter())
            .map(|(a, b)| a * b)
            .collect();
        let c = ntt(&c_inv, &root_inv);

        let t_inv = ModInt::new(t).inv();
        c.into_iter().take(s).map(|x| x * t_inv).collect()
    }

    fn ntt(a: &Vec<ModInt>, root: &Vec<ModInt>) -> Vec<ModInt> {
        let n = a.len();
        let d = n.trailing_zeros();

        let mask = n - 1;
        let mut res = a.clone();

        for i in (0..d).map(|i| (n - 1) >> i + 1) {
            res = (0..n)
                .map(|j| {
                    let l = i & j;
                    let u = j ^ l;
                    let s = u << 1 & mask;
                    res[s | l] + root[u] * res[s | i + 1 | l]
                })
                .collect();
        }

        res
    }
}
