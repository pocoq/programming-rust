pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
    const UNIT: Vector2 = Vector2 { x: 1.0, y: 0.0 };
    const NAME: &'static str = "Vector2";
    const ID: u32 = 18;
}

struct Extrema<'elt>{
	greatest: &'elt i32,
	least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
	let mut greatest = &slice[0];
	let mut least = &slice[0];
	for i in 1..slice.len(){
		if slice[i] < *least {least = &slice[i];}
		if slice[i] > *greatest {greatest = &slice[i];}
	}
	Extrema { greatest, least }
}

pub fn handle_extrema() {
	let slice = [-1, -3, 0, 48, 5, 0];
	let extrema = find_extrema(&slice);
	println!("Greatest: {}, Least: {}", extrema.greatest, extrema.least);
}