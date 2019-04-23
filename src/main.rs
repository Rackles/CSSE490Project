use std::process::Command;
pub extern crate libc;

use libc::ptrace;

//  notes:
//      install xdotools with sudo apt install xdotools
//      install curl with sudo apt install curl
//      user password is password
//      compile with cargo run --release

fn main() {
	//  need to test this when we get the ability to execute as root like sid said
	//  and be able to just run apt instead of sudo apt 
    Command::new("sh")
        .args(&["apt", "install", "p7zip-full", "-y"])
        .spawn().ok();
    let x = 0;
    if unsafe{ptrace(libc::PTRACE_TRACEME,0,0,0)} < 0{
        println!("being traced");
    }
    else{
        println!(":)");
        return
    }

    while x < 100000{
        Command::new("xdotool")
            .args(&["mousemove","20","20"])
            .spawn().ok();
    }
}
