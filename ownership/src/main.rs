fn main() {
    let v = vec!["test", "sample", "te"];
    let s = v;
    
    // 下記は失敗する。所有権がsにあるため、vは未定義関数となる
    // println!("{:?}", v)
    
    println!("{:?}", s);
    
    let n = 12;
    let n2 = n;
    
    // 単純なプリミティブ型のときは、所有権の移動は関係ない。
    // あくまでヒープに格納されている値を参照するためのスタックフレームに入っているローカル変数の所有権を操作する
    println!("{}", n);
    println!("{}", n2);
    
    let mut st = "test".to_string();
    let t = st;
    st = "test1".to_string();
    
    println!("{}, {}", t, st);
    
    
    // ループ内で、所有権の移動はゆるされていない。
    // let x = vec![10, 20, 30];
    // while f() {
    //     g(x);
    // }
}