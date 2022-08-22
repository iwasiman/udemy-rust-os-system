pub fn run() {
    // 10 Stack overflow Vector型

    // Rustではスタックは最大8MBときまっている。
    //let _a1: [u8;7000000] = [1; 7000000]; // 7MB使う
    //let _a1: [u8;9000000] = [1; 9000000]; // 9MB使うと...
    /*

    thread 'main' has overflowed its stack
    fatal runtime error: stack overflow
    Abort trap: 6
    */

    // ベクタ型
    let mut _v1 = vec![1, 2, 3, 4];
    let _v2 = vec![5, 6, 7, 8];
    let mut _v3 = vec![9, 10];
    println!("スタックアドレス _v1 {:p}", &_v1); //x7ff7be7eee60
    println!("スタックアドレス _v2 {:p}", &_v2);
    println!("ヒープメモリアドレス _v1 {:p}", _v1.as_ptr()); //0x7ff66a004280
                                                             /*
                                                             ベクタ型のメモリの持ち方はString型とほぼ同じ。capがバイト数でなく要素数になる。
                                                             ptr 先頭の番地だが、型の情報も持ってる
                                                             len 4
                                                             cap 4

                                                             */
    println!("LenとCapacity  _v1 {} {}", _v1.len(), _v1.capacity()); // 4 4
    _v1.insert(1, 10);

    /*
    Boxポインタ のメモリ表現

    */
    let _t1 = (10, String::from("hello"));
    println!("スタックアドレス _t1 {:p}", &_t1); //0x7ff7b9380f00
    println!("ヒープメモリアドレス _t1.1 {:?}", _t1.1.as_ptr()); //00x7fb7ad705d10
    println!("lenとcapacity _t1.1 {} {}", _t1.1.len(), _t1.1.capacity()); // 5, 5
                                                                          // Boxポインタで中身を書き換える
    let mut _b1 = Box::new(_t1);
    (*_b1).1 += "world";
    println!("{} {}", _b1.0, _b1.1); // 10 helloworld

    // このBoxポインタの話は難しかった。。。
}
