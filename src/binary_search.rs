fn binary_search_template(k: usize) {
    let mut lower = 0;
    let mut upper = 1 << 60;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(med, k) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(med: usize, k: usize) -> bool {
    true
}

fn binary_search_template_f64(a: Vec<i64>) {
    let mut lower = 0.0;
    let mut upper = 1e18;
    while upper - lower > 1e-5 {
        let med = (lower + upper) / 2.0;
        if is_ave_over(&a, med) {
            lower = med;
        } else {
            upper = med;
        }
    }
}

fn is_ave_over(a: &[i64], k: f64) -> bool {
    let a = a.iter().map(|&a| a as f64 - k).collect::<Vec<_>>();
    a.iter().sum::<f64>() > 0.0
}

fn is_med_over(a: &[i64], k: i64) -> bool {
    // a>=kなら1、そうでないなら-1
    let a = a
        .iter()
        .map(|&a| if a >= k { 1 } else { -1 })
        .collect::<Vec<_>>();
    a.iter().sum::<i32>() > 0
}
