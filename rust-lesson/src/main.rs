mod debug;
mod enums;
mod error_handling;
mod generics;
mod lifetime;
mod ownewship;
mod stack_heap;
mod structs;
mod traits;
mod unit_test;
mod vars; //varsモジュールを使いますよ宣言

fn main() {
    //println!("Hello, world!");
    vars::run();
    // vars::sub_a::func_a();
    stack_heap::run();
    ownewship::run();
    generics::run();
    lifetime::run();
    structs::run();
    enums::run();
    traits::run();
    error_handling::run();
    unit_test::run();
    debug::run();
}
