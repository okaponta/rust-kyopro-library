fn shakutori_template(n: usize, k: usize, a: Vec<usize>) {
    // 半開区間
    let mut left = 0;
    let mut right = 0;
    let mut sum = 0;
    let mut ans = 1 << 60;
    loop {
        while sum < k && right < n {
            sum += a[right];
            right += 1;
        }
        if sum < k {
            // しゃくとり終了
            break;
        }
        ans = ans.min(right - left);
        sum -= a[left];
        left += 1;
    }
}
