pub mod ch09_structs;
pub mod ch10_enums;

use ch09_structs::{node, queue};
use ch10_enums::time_units;

use crate::{ch09_structs::vector, ch10_enums::planets};
fn main() {
    //    Chapter 09 - Structs
    println!("Chaper 09 - Structs ");
    println!("* Queue example *");
    queue::handle_queue();

    println!("* Generic Queue *");

    queue::handle_generic_queue();
    println!("* Node example *");

    node::handle_node();
    println!("* Extrema example *");

    vector::handle_extrema();

    println!("-------------------------------------------------------");
    //    Chapter 10 - Enums
    println!("Chapter 10 - Enums ");
    println!("* Rough time example *");

    time_units::handle_rough_time();
    println!("* Shape example *");

    time_units::handle_shape();
    println!("* Planets example *");

    planets::handle_planets();
}
