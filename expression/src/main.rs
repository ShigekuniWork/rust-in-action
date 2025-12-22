fn main() {
    assert_eq!(calc(3, 5, "+"), 8);
    assert_eq!(calc(3, 5, "aaaa"), 15);

    print_message(0);
    let v = 12_i32.checked_pow(3);
    if let Some(value) = v {
        println!("{}", value);
    };

    loop_num();
    iterable_num(20);
    error_message();

    let n = 3.14;
    println!("{}", n as i32);

    let is_even = |n: i32| n % 2 == 0;
    if is_even(n as i32) {
        println!("even")
    } else {
        println!("odd")
    }
}

fn calc(num1: i32, num2: i32, operator: &str) -> i32 {
    let res = if operator == "+" {
        num1 + num2
    } else {
        num1 * num2
    };

    return res;
}

fn print_message(num: i32) {
    match num {
        0 => println!("OK"),
        1 => println!("Wires Tangled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", num),
    }
}

fn loop_num() {
    let mut i: i32 = 2;
    while let Some(v) = i.checked_pow(2) {
        println!("{}", v);
        i = i + v
    }
}

fn iterable_num(num: i32) {
    let mut sum = 0;
    for i in 0..num {
        sum += i;
        println!("{}", sum);
    }
}

fn error_message() {
    let mut message = vec![
        "error1".to_string(),
        "error2".to_string(),
        "error3".to_string(),
    ];
    for m in &message {
        println!("{}", m);
    }
    println!("{} error", message.len());

    for m in &mut message {
        m.push_str("\n");
        println!("{}", m);
    }
}
