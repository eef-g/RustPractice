use std::collections::HashMap;


pub fn hashmap_example() {
    let mut reviews: HashMap<String, String> = HashMap::new();
    // Inserting a value as a key, value pair.
    reviews.insert(String::from("Ancient Roman History"), String::from("Very accurate."));
    reviews.insert(String::from("Cooking with Rhubarb"), String::from("Sweet recipes."));
    reviews.insert(String::from("Programming in Rust"), String::from("Great examples."));

    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    // Remove something from a hashmap
    let obsolete: &str = "Ancient Roman History";
    println!("\nRemoving \'{}\'.", obsolete);
    reviews.remove(obsolete);

    // Confirm that it was removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}

pub fn loop_example() {
    // This is a for loop
    for i in 0..5
    {
        println!("i = {}", i);
    }

    // This is a while loop
    let mut j: i32 = 0;
    while j < 5
    {
        println!("j = {}", j);
        j += 1;
    }

    // This is a foreach loop
    let boids = ["Starling", "Robin", "Duck", "Owl", "Goose"];
    for boid in boids.iter()
    {
        println!("{}!", boid);
    }
}