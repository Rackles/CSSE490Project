use std::process::Command;
pub extern crate libc;

use libc::ptrace;

//  notes:
//      install xdotools with sudo apt install xdotools
//      install curl with sudo apt install curl
//      user password is password
//      compile with cargo run --release

fn main() {
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
