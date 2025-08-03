use std::collections::HashSet;
pub fn get_default() {
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & (n - 1) == 0);
    println!("Powers of two: {:?}", powers_of_two);
    println!("Impure: {:?}", impure);
    println!("Default value for i32: {}", 16 & 15);

    let (upper, lower) = "Hello, World!"
        .chars()
        .partition::<String, _>(|c| c.is_uppercase());
    println!("Uppercase: {}, Lowercase: {}", upper, lower);
}
