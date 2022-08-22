pub fn run() {
    println!("");
    println!("ジェネリクス-------------------");

    let _number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    //println!("largestなのは {} ", largest);
    println!("largestなのは {} ", largest_i32(_number_list));

    // Rustは''で囲うとchar型になる。
    let chara_list = vec!['a', 'b', 'c', 'd'];
    let _number_list2 = vec![34, 50, 25, 100, 65];
    println!(
        "genericsな関数を使う largestなのは {} ",
        largest(_number_list2)
    ); //100
    println!(
        "genericsな関数を使う largestなのは {} ",
        largest(chara_list)
    ); // d

    // 構造体にジェネリクス
    let _p1 = Point { x: 1, y: 2 };
    let _p2 = Point { x: 1.0, y: 2.0 };
    let _p3 = Point { x: 5, y: 10.4 };
    let _p4 = Point {
        x: "Rust",
        y: "Cool!",
    };
    println!("構造体にジェネリクス {} {}", _p4.x, _p4.y); //

    //let p5 = _p3.mixup(_p4);

    /*
    Rust のメモリ安全性
    二重解放エラー　所有権システム
    メモリ解放忘れ ->RAII Resource Aquisition Is Initialization
    ダングリングポインター -> ライフタイム

    RAII: 変数初期化時にリソース(メモリ)が確保される。
    そしてスコープを抜けるとリソースが開放される。物理ファイル、ネットワーク接続も

        */
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// トレイト境界を使ってTの範囲を絞る。
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

// struct PointAnother<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> PointAnother<T, U> {
//     fn mixup<V,W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
//         PointAnother {
//              x: self.x,
//              y: other.y
//         }
//     }
// }
