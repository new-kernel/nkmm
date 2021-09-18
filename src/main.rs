#[cfg(target_os = "novusk")]
compile_error!("Sorry! Novusk can't compile nkmm");

use std::env;
use std::process::exit;

fn main() {
    let env_args = env::args().collect();
    let args: Vec<String> = env_args;

    if args[1] == "build" {
        println!("Building Novusk Kernel Module...");
    } else if args[1] == "run" {
        if args[2] != "" {
            panic!("Not a valid module");
        }
        unimplemented!();
    } else {
        println!("{} - Not an argument, use \"build\" or \"run\"", args[1]);
        exit(0);
    }
}

