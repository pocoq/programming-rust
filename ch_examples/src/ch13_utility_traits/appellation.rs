struct Appelation{
	name: String,
	nicknames: Vec<String>,
}

impl Drop for Appelation{
	fn drop(&mut self){
		print!("Dropping {} ", self.name);
		if !self.nicknames.is_empty(){
			print!("AKA {}", self.nicknames.join(", "));
		}
		println!();
	}
}

pub fn get_appelation(){
	let mut a = Appelation{
		name: "Bingo".to_string(),
		nicknames: vec!["Go".to_string(), "Go Go".to_string()],
	};
	println!("Before assignment:");
	a = Appelation{
		name: "Michel".to_string(),
		nicknames: vec![],
	};
	println!("At the end of block");
}


