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

// a[i]-a[i+k]の最小値を出力する
fn slide_min(a: Vec<usize>, n: usize, k: usize) -> Vec<usize> {
    let mut q = std::collections::VecDeque::new();
    let mut ans = vec![];
    for i in 0..n {
        while let Some(b) = q.pop_back() {
            if a[b] < a[i] {
                q.push_back(b);
                break;
            }
        }
        q.push_back(i);
        if k <= i + 1 {
            let f = q.pop_front().unwrap();
            ans.push(a[f]);
            if f != i + 1 - k {
                q.push_front(f);
            }
        }
    }
    ans
}
