pub fn run() {
    println!("");
    println!("Enum -------------------");
    let linux = OS::Linux(1991, String::from("Linux"));
    let windows = OS::Windows(1985, String::from("Microsoft"));
    let mac = OS::Mac(2001, String::from("Apple"));
    print_os_info(linux);
    print_os_info(windows);
    print_os_info(mac);
}

enum OS {
    Windows(u32, String),
    Mac(u32, String),
    Linux(u32, String),
}

fn print_os_info(os: OS) {
    match os {
        OS::Windows(year, who) => {
            println!("Widowsのリリースは {} だれかな {}", year, who);
        }
        OS::Linux(year, who) => {
            println!("Linuxのリリースは {} だれかな {}", year, who);
        }
        OS::Mac(year, who) => {
            println!("Macのリリースは {} だれかな {}", year, who);
        }
    }
}
