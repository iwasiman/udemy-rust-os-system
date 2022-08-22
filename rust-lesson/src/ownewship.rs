pub fn run() {
    println!("所有権・参照・借用-------------------");

    let _s1 = String::from("hello");
    let _s2 = _s1;
    //println!("{} {}", _s1, _s2); できない
    // borrow of moved value: `_s1`
    // value borrowed here after move

    // コピーされる場合
    let _i1 = 1;
    let _i2 = _i1;
    println!(
        "整数型はcopyトレイトを実装してるからコピーできるよ {} {}",
        _i1, _i2
    );
    println!("アドレス {:p}", &_i1); // 2つは違う場所
    println!("アドレス {:p}", &_i2);
    let _sl1 = "りてらる";
    let _sl2 = _sl1;
    println!("{} {}", _sl1, _sl2);
    println!(
        "スライスもcopyトレイトを実装してるからコピーできるよ {} {}",
        _sl1, _sl2
    );
    println!("_sl1 アドレス {:p}", &_sl1); // 2つは違う場所
    println!("_sl2 アドレス {:p}", &_sl2);

    /*
    // deep copy
    スタックにあるs1が、ヒープ領域にある helloをさす
    スタックにあるs2が、ヒープ領域に helloをさす 別の場所。

    */
    let _s3 = String::from("hello");
    let _s4 = _s3.clone();
    println!("アドレス _s3 {:p}", &_s3); // 2つは違う場所
    println!("アドレス _s4 {:p}", &_s4);
    println!("ヒープのアドレス _s3 {:p}", _s3.as_ptr()); // 2つは違う場所
    println!("ヒープのアドレス _s4 {:p}", _s4.as_ptr());
    println!("cloneしてるから所有権は移動しないよ {} {}", _s3, _s4);

    let _s5 = String::from("hello"); // len: 5 cap: 5
    take_ownership(_s5);
    //println!("{}", _s5); //borrowしてるからだめ take_ownership が終わった時点でdropされてしまう。

    let _s6 = String::from("hello");
    println!("スタックアドレス _s6 {:p}", &_s6); // _s7と違うアドレス。移動後はもう使えない。
    println!("ヒープのアドレス _s6 {:p}", _s6.as_ptr());
    println!("LenとCap _s6 {} {}", _s6.len(), _s6.capacity()); // 5, 5
    let _s7 = take_giveback_ownership(_s6);
    println!("スタックアドレス _s7 {:p}", &_s7);
    println!("ヒープのアドレス _s7 {:p}", _s7.as_ptr()); // _s6と_s7で同じ場所を指してる
    println!("LenとCap _s7 {} {}", _s7.len(), _s7.capacity()); // 5,5

    // 参照を使う。参照は複数できる。
    let _len = calculate_length(&_s7);
    println!("参照を使う関数でLen {} {}", _s7.len(), _len); // 5, 5

    let _s10 = String::from("hello");
    let r1 = &_s10;
    let r2 = &_s10;
    println!("1つの変数に参照は複数回できるよ {} {} {}", _s10, r1, r2); // hello hello hello

    // ミュータブルな参照とイミュータブルな参照は同時にはできない。
    let mut _s10 = String::from("hello");
    //let r1 = &_s10;
    //let r2 = &mut _s10;
    //cannot borrow `_s10` as mutable because it is also borrowed as immutable
    //mutable borrow occurs here
    //println!("{} {}", r1, r2);

    let mut _s11 = String::from("hello");
    let _r1 = &mut _s11;
    //println!("{}", _s11); // _r1に所有権が移っているのでここでは使えない。
    println!("{}", _r1);
    println!("{}", _s11); // _r1のリファレンスが有効なのは上の行までなので次の行だとできる。

    let mut _s12 = String::from("hello");
    let r1 = &_s12;
    let r2 = &_s12;
    println!("1つの変数に参照は複数回 {} {} {}", _s12, r1, r2); //

    /*
    変数宣言したライフタイム:
    let mut s11 = String::from("a"); //その変数が最後に使われたところか、スコープが終わったところで終わり
    let r1 = &mut s11;
    println!("{}", r1); // 参照で使うとここでドロップ
    println!("{}", s11); // 最後に使用、ここでドロップ

    ダングリングポインタ
    pub fn run() {
        let r;
        {
            let x = 5;
            r = &x; // x's ptr is bind to r x dropped here
        }
        println!("r: {}", r);
    }
    参照は実体より長生きしてはいけない。
    コンパイラが全てチェックする。


         */
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
