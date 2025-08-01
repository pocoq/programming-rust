pub mod ch09_structs;
pub mod ch10_enums;
pub mod ch11_traits;
pub mod ch12_operator_overload;

use ch09_structs::{node, queue, vector};
use ch10_enums::{planets, time_units};
use ch11_traits::{files, generic};
use ch12_operator_overload::{complex, interval};

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

    println!("* Patterns example *");
    ch10_enums::patterns::handle_patterns();

    println!("-------------------------------------------------------");
    //    Chapter 11 - Traits
    println!("Chapter 11 - Traits ");

    println!("* Files example *");
    files::write_to_file();

	println!("* Generic Dot Product example *");
	generic::calculate_dot_product();
    println!("-------------------------------------------------------");

	//    Chapter 12 - Operator Overloading
	println!("Chapter 12 - Operator Overloading ");

	println!("* Complex number example *");
	complex::handle_complex();

	println!("* Interval example *");
	interval::handle_interval();
}
