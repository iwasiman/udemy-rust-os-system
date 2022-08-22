pub fn run() {
    println!("");
    println!("構造体 -------------------");

    let _user1 = User {
        username: "aaa".to_string(),
        email: "aaa@aaa".to_string(),
        sign_in_count: 1,
        active: true,
    };
    println!(
        "構造体をデバッグ出力 {:#?} {} {} {} {}",
        _user1, _user1.username, _user1.email, _user1.sign_in_count, _user1.active
    );

    let rect = Rectangle::create(20, 20);
    println!("Rectangleを表示 {:#?} {} {}", rect, rect.height, rect.width);
    println!("Rectangleのメソッドareaを使う");
    rect.area();
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // 通常newと聞いたがこの講習だと違った。
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) {
        println!("面積 {}", self.height * self.width);
    }
}
