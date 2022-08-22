pub fn run() {
    println!("");
    println!("Generic lifetime annotation -------------------");
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("もどりち {}", res1); //y

    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4); //ダングリングポインタになるのでコンパイラが教えてくれる
                                        //println!("もどりち {}", res2); //y

        //`st4` does not live long enough
        //borrowed value does not live long enough
        println!("もどりち {}", res2); //
    }
    //println!("もどりち {}", res2); //だめ
}

// 引数2つのうち短い方のライフタイムを、戻り値に使う
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
