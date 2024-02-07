mod register;
mod cpu;

use cpu::SM83;

fn main() {
    println!("Hello, world!");

    let mut sm83 = SM83::new();
    sm83.fetch_execute();
}
