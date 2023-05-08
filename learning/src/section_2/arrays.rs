pub fn array_example() {
    let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"]; // Make a normal array
    let bytes = [0; 5]; // This creates an array of 5 zeroes

    let first = days[0]; // Index arrays like you do with most languages.
    let last = days[6];

    // Print out examples of the arrays
    println!("The first day of the week is {}", first);
    println!("The last day of the week is {}", last);
    println!("The 'bytes' array has {} elements", bytes.len());
}