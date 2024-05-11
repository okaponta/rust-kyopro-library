use std::collections::HashMap;

use proconio::input_interactive;

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

fn lower_bound_f64(v: &Vec<f64>, t: f64) -> usize {
    let mut lower = 0;
    let mut upper = v.len();
    while upper != lower {
        let med = (lower + upper) / 2;
        if v[med] < t {
            lower = med + 1;
        } else {
            upper = med;
        }
    }
    lower
}

// 三分探索(インタラクティブのメモ化)
fn three_part_search(n: usize) {
    fn f(i: usize, map: &mut HashMap<usize, usize>) -> usize {
        if map.contains_key(&i) {
            return map[&i];
        }
        println!("? {}", i);
        input_interactive!(a:usize);
        map.insert(i, a);
        a
    }
    let mut map = HashMap::new();
    let mut lower = 1;
    let mut upper = n;
    while upper - lower > 2 {
        let med1 = (lower * 2 + upper) / 3;
        let med2 = (lower + upper * 2) / 3;
        if f(med1, &mut map) < f(med2, &mut map) {
            lower = med1;
        } else {
            upper = med2;
        }
    }
    println!(
        "! {}",
        f(lower, &mut map)
            .max(f(upper, &mut map))
            .max(f((lower + upper) / 2, &mut map))
    );
}
