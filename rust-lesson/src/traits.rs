pub fn run() {
    println!("");
    println!("構造体 -------------------");
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);

    let tweet = Tweet {
        username: String::from("うまのほん"),
        content: String::from("なんか"),
        replay: false,
        retweet: false,
    };
    println!("{} {}", tweet.replay, tweet.retweet);
    println!("あたらしいついーと {}", tweet.summarize());
}

trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}

struct Banana;
impl Fruits for Banana {
    // fnまで打つと候補が出る。素晴らしい。
    fn price(&self) -> u32 {
        5
    }
}

fn get_price<T: Fruits>(fruits: T) {
    println!("price is: {}", fruits.price());
}

struct Tweet {
    username: String,
    content: String,
    replay: bool,
    retweet: bool,
}
trait Summary {
    fn summarize(&self) -> String {
        // デフォルト実装 構造体側でオーバーライドもできる。
        String::from("(Read more...)")
    }
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
trait Message {}

// 引数は2つのトレイトを実装してるものだけ,という指定もできる。使用時に引数に満たさない値を入れるとコンパイルエラー。
// fn notify_another(item: &(impl Summary + Message)) {

// }
