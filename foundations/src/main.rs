fn main() {
    // number
    assert_eq!(2_i32.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
    
    assert_eq!(2_i32.checked_pow(4), Some(16));
    assert_eq!(100_u8.checked_add(200), None);
    assert_eq!((-4_i32).checked_abs(), Some(4));
    assert_eq!((-128_i8).checked_div(-1), None);
    
    // bool
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}
