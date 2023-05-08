pub mod section_1; // This includes the folder "section_1", which needs a "mod.rs" file to be loaded
// This "mod.rs" file contains references to the other files in the folder.
pub mod section_2;
pub mod section_3;
pub mod section_4;
pub mod section_5;

fn main()
{
    // This will show us basic Rust stuff as well as a car factory example
    section_1::showcase();

    // Section 2 will go here
    section_2::main();

    // Section 3
    section_3::section3_showcase();

    // To throw an error, use the 'panic!()' macro
    // panic!("This is an error message!");
    section_4::examples();

    section_5::example();
}