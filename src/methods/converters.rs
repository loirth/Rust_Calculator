use std::io;

pub fn float_from_input(what_number: String) -> f64 {
	loop {
		println!("Enter the {} number:", what_number);
		
		let mut number: String = String::new();

		io::stdin()
		.read_line(&mut number)
		.expect("Failed to read line");

		let _some_number = match number.trim().parse::<f64>() {
				Ok(_some_number) => {
					break _some_number
				}
				Err(_) => continue
		};
	}
}