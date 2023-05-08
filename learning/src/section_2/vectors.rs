pub fn vectors_example()
{
    // A vector: a dynamic array <- But only if you make it a 'mut' type
    let three_nums = vec![15, 3, 46]; // Create a vector with 3 numbers

    let mut fruit = Vec::new(); // Create a new vector that's mutable
    fruit.push("Apple"); // Add an element to the vector
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit); // Print out the vector <- This shows the vector as a list in the console

}