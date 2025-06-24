mod node;
mod queue;

fn main() {
    queue::handle_queue();
    println!("Queue example completed successfully.");

    node::handle_node();
    println!("Node example completed successfully.");
}
