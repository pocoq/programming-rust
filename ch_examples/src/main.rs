pub mod ch09_structs;

use ch09_structs::{node, queue};
fn main() {
    //    Chapter 09 - Structs

    queue::handle_queue();
    println!("Queue example completed successfully.");

    node::handle_node();
    println!("Node example completed successfully.");
}
