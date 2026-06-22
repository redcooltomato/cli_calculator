use crate::calculate_expression;

#[test]
fn basic_arithmetic() {
    let t1 = "2 + 3 * 4 - 6 / 1.5".to_string();
    assert_eq!(calculate_expression(&t1).unwrap(), 10 as f64);

    let t2 = "-2 * 4".to_string();
    assert_eq!(calculate_expression(&t2).unwrap(), -8 as f64);
}

#[test]
fn powers() {
    let t1 = "sqrt(3 rt 64 ^ 2)".to_string();
    assert_eq!(calculate_expression(&t1).unwrap(), 4 as f64);

    let t2 = "-3 ^ 3".to_string();
    assert_eq!(calculate_expression(&t2).unwrap(), -27 as f64);

    let t3 = "-12 ^ 2".to_string();
    assert_eq!(calculate_expression(&t3).unwrap(), 144 as f64);
}

#[test]
fn trigonometry() {
    let t1 = "sin 30 + cos 60 - tan 45 + asin 0.5 - acos 0.5 + atan 1".to_string();
    assert!((calculate_expression(&t1).unwrap() - (14.9 as f64)).abs() <= 0.1 as f64); // decimal accuracy
}