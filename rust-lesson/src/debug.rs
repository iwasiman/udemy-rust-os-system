pub fn run() {
    let mut _x = 1;
    _x = 2;
    _x = 3;
    {
        let mut _y = 1;
        _y = 2;
        _y = 3;
    }
    _x = 4;
    _x = 5;
}
// _x = 5 の行を抜けると、デバッガのVariablesから_xもなくなる。ドロップするkら。
