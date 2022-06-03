pub fn is_real_num(num: &str) -> bool {
    match num.trim().parse::<f64>() {
        Ok(_) => true,
        Err(_) => false,
    }
}
