// 現在hの高さで静止している物体が重力加速度gの場合にt秒後にいる高さ
fn free_fall(t: f64, h: f64, g: f64) -> f64 {
    let period = ((2.0 * h) / g).sqrt();
    let shift = ((t / period).floor() / 2.0).ceil() * 2.0;
    h - (g / 2.0) * (t - shift * period).powi(2)
}
