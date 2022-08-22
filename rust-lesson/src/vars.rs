pub mod sub_a;
pub mod sub_b;

const _MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars moduleだよ");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut _x = 5;
    //x = 6; // immutableでエラー
    _x = 6;

    let _i1 = 3; //デフォルトで i32型になる。
    let _f1 = 0.1; // 先頭アンダーバーだと未使用で警告が出ない。

    println!("{}", usize::BITS); // 64
    println!("メモリアドレス 定数_MAX_POINTS {:p}", &_MAX_POINTS); //0x102ec039c  静的領域

    let _i2: i64 = 1; //64ビットなので8バイトになる
    let _i3: i64 = 2; //64ビットなので8バイトになる
    println!("メモリアドレス _i2 {:p}", &_i2); //0x7ff7bd176ee0
    println!("メモリアドレス _i3 {:p}", &_i3); //0x7ff7bd176ee8 8バイト分後ろに格納される

    // シャドーイング
    let _y = 5;
    println!("メモリアドレス _y {:p}", &_y); //x7ff7b6d62e94
    let _y = _y + 1;
    println!("メモリアドレス _y {:p}", &_y); //0x7ff7b6d62ee4  スタック上に別領域に入る
    let _y = _y * 2;
    println!("メモリアドレス _y {:p}", &_y); //0x7ff7b6d62f34
    {
        let _y = 0;
        println!("_yの値 {}", _y); //このスコープでは0になる。
    }

    // タプル
    let _t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = _t1; // 分割して代入できる

    let mut _t2 = ((0, 1), (2, 3));
    let ((ref mut _x1_ptr, ref mut _y1_ptr), _) = _t2;
    *_x1_ptr = 5; // * をつけるとアドレスの指す先を書き換えられる。
    println!("{:?}", _t2);

    // 配列
    let _a1 = [1, 2, 3, 4, 5];
    let _a2 = [0; 10];
    println!("{:?} {:?} {} {}", _a1, _a2, _a1[2], _a1[3]);

    slice_string();
    owner();
}

fn slice_string() {
    // 文字列とString

    let _s1 = "helloこんにちは挨拶"; //26bytes 7*3 + 5*1
    let _s2 = "hello";
    // スライスは、先頭アドレスと要素数で表現される
    println!("スタックアドレス _s1 {:p}", &_s1); // 0x7ff7b56559f0
    println!("スタックアドレス _s2 {:p}", &_s2); // 0x7ff7b5655a00
    println!("静的メモリアドレス _s1 {:p}", _s1.as_ptr()); // 0x1001cc1f0
    println!("静的メモリアドレス _s2 {:p}", _s2.as_ptr()); // 0x1001cc20a

    // String型
    let mut _s1 = String::from("hello");
    println!("長さ_s1 {:p}", &_s1.len()); // 0x7ff7b56559f0
}

fn owner() {
    /*
    所有権による二重解放エラー回避

    所有権があるのはデータに対して必ずひとり。
    C/C++は所有権者がメモリを解放してた　ー＞ 解放(drop)はRustによって自動的に行われる。

    String
    let mut s1 = String::from("hello");
    ptr (8bytes) : ヒープ領域の最初の1文字目のアドレスを持つ
    len (8bytes) :5
    cap (8bytes): 5

    文字列スライスは所有権が適用されない。
    文字列リテラルからスライスを作ると静的領域にあるので、秋法する必要がない。
    String型から文字列スライスを作ると、所有権は移動せず借用になる。


    */
}
