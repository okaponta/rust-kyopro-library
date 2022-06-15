fn print(n: usize, f: f64) {
    // 0埋め(左うめ3桁)
    println!("{:<03}", n);
    // f64の小数点以下桁数指定(以下は小数点2桁以下切り捨て)
    println!("{:.*}", 2, (f * 100.0).floor() / 100.0);
    // 2進数
    println!("{:b}", n);
    // 8進数
    println!("{:o}", n);
    // 16進数
    println!("{:x}", n);
}
