pub mod ch09_structs;

use ch09_structs::{node, queue};

use crate::ch09_structs::vector;
fn main() {
    //    Chapter 09 - Structs

    queue::handle_queue();
    println!("Queue example completed successfully.");

	queue::handle_generic_queue();
	println!("Generic Queue example completed successfully.");

    node::handle_node();
    println!("Node example completed successfully.");

	vector::handle_extrema();
	println!("Extrema example completed successfully.");
}
