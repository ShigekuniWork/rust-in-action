fn main() {
    let twenty = 10;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_001;
    println!("{}", one_million.pow(2));

    let total = addition + one_million.pow(2) as i32;
    // i64からi32に縮めたことによって、下位32bitだけが残される。
    // この場合は、32bit目が1で入れ替わってしまうため、-として答えが出る。
    println!("{}", total);

    let forty_twos = [42.0, 42f32, 42.0_f32];

    println!("{}", forty_twos[0]);
}
