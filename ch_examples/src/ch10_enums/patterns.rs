fn describe_point(x: i32, y: i32) -> &'static str {
    use std::cmp::Ordering::*;

    match (x.cmp(&0), y.cmp(&0)) {
        (Equal, Equal) => "at the origin",
        (_, Equal) => "on the x-axis",
        (Equal, _) => "on the y-axis",
        (Greater, Greater) => "in the first quadrant",
        (Less, Greater) => "in the second quadrant",
        _ => "somewhere else",
    }
}

fn greet_people(names: &[&str]) {
    match names {
        [] => println!("Hello, nobody"),
        [a] => println!("Hello, {}", a),
        [a, b] => println!("Hello, {} and {}", a, b),
        [a, .., b] => println!("Hello, from {} to {}", a, b),
    }
}

pub fn handle_patterns() {
    let point = (1, 2);
    let description = describe_point(point.0, point.1);
    println!("The point ({}, {}) is {}", point.0, point.1, description);

    let names = ["Alice", "Charlie"];
    greet_people(&names);
}
