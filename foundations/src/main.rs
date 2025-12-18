use std::vec;

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

    // char
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);

    // tuple
    let self_introduction = "My name is koki shigekuni";
    let temp = self_introduction.split_at(21);
    let head = temp.0;
    let tail = temp.1;

    assert_eq!(head, "My name is koki shige");
    assert_eq!(tail, "kuni");

    // array
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // vector
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");

    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut cap: Vec<i32> = Vec::with_capacity(2);
    assert_eq!(cap.capacity(), 2);
    cap.push(2);
    assert_eq!(cap.capacity(), 2);

    assert_eq!(cap.pop(), Some(2));
}
