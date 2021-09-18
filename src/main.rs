#[macro_use] extern crate cfg_if;

cfg_if! {
    if #[cfg(target_os = "novusk")] {
        #![no_std]
        #![no_main]
    }
}

#[cfg(not(target_os = "novusk"))]
fn main() {
    println!("Hello, world!");
}


