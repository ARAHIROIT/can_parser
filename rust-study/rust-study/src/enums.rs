enum OS {
    windows(u32, String),
    Mac(u32, String),
    Linux(u32,String,)
}



pub fn run(){
    let windows = OS::windows(1985, String::from("windows"));
    print_os_info(windows);
    let Mac: OS = OS::Mac(2001, String::from("Mac"));
    print_os_info(Mac);

    let Linux = OS::Linux(1991, String::from("Linux"));
    print_os_info(Linux);

}
fn print_os_info(os: OS){
    match os {
        OS::windows(year, who ) => {
            println!("windows {} {}", year, who);
        }
        OS::Mac(year, who) => {
            println!("Mac {} {}", year, who);
        }
        OS::Linux(year, who) => {
            println!("Linux {} {}", year, who);
        }
    }

}