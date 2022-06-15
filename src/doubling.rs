// https://atcoder.jp/contests/abc167/tasks/abc167_d
fn doubling_template(mut k: usize, mut a: Vec<usize>) {
    let mut ans = 0;
    // 普通にkを変える方法じゃなくて、桁数でぐるぐるまわすでもOK
    while k > 0 {
        if k & 1 > 0 {
            ans = a[ans];
        }
        a = a.iter().map(|&i| a[i]).collect();
        k >>= 1;
    }
    println!("{}", ans + 1);
}
