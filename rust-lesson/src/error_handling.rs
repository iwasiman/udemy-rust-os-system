pub fn run() {
    println!("");
    println!("エラーハンドリング-------------------");

    let res1 = division_option(5.0, 2.0);
    match res1 {
        Some(x) => println!("結果は {:.3}", x), // 2.550
        None => println!("Not allowed"),
    }
    let res2 = division_result(5.0, 0.0);
    match res2 {
        Ok(x) => println!("結果は {:.3}", x),
        Err(e) => println!("{}", e), // エラーだよ
    }

    // 配列に？を使う例
    let a = [0, 1, 2];
    let res3 = sum(&a);
    match res3 {
        Some(x) => println!("合計は {}", x), // こちらにくる
        None => println!("out of index　だよ"),
    }
    let a2 = [0, 1];
    let res4 = sum(&a2);
    match res4 {
        Some(x) => println!("合計は {}", x),
        None => println!("out of index　だよ"), // こちらにくる
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err("えらーだよ".to_string())
    } else {
        Ok(x / y)
    }
}

fn sum(a: &[i32]) -> Option<i32> {
    // 配列の範囲外にアクセスしても、？をしておくとそこでNoneを返してくれる。すごい。
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}
