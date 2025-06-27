mod macro_e;
mod poc;

pub fn main() {
    println!("proof of concept example: ");
    crate::poc::example();

    println!("");

    println!("macro example: ");
    crate::macro_e::example();
}
