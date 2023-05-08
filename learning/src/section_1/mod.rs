pub mod types;
pub mod cars;

pub fn main() {
    types::types_showcase();
    cars::cars_showcase();
}

pub fn showcase() {
    println!("\n\n-=| Section 1 Showcase |=-\n");
    main();
    println!("-=-=-=-=-=-=-=-=-=-=-=-=\n");
}