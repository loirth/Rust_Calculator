use std::io;

fn main() {
	introduction();

	loop {
		println!("What you wanna do? (+, -, *, /, **, %, 0 - exit)");

		let mut some_do: String = String::new();

		io::stdin()
			.read_line(&mut some_do)
			.expect("Failed to read line");

		println!("Enter the first number");

		let first_number = int_from_input(String::new());

		println!("Enter the second number");

		let second_number = int_from_input(String::new());

		let result: i64 = calc(first_number, second_number, some_do.clone());

		println!("The result is: {}\n{} {} {} = {}", result, first_number, some_do, second_number, result);
	}
}

fn introduction() {
	println!("             Hi!               ");
	println!("Welcome to the Rust Calculator!");
	println!("      What you wanna do?       ");
	println!("0 - Start/continue | 1 - Help | 2 - Exit");

	let mut start_command: String = String::new();

	loop {
		io::stdin()
			.read_line(&mut start_command)
			.expect("Failed to read line");

		let start_command = match start_command.trim().parse() {
			Ok(start_command) => start_command,
			Err(_) => {
				println!("Enter the number!");
				continue;
			},
		};
		
		match start_command {
			0 => {
				println!("Start");
				break;
			},
			1 => {
				println!("Help");
				break;
			},
			2 => {
				panic!("You exit the rust_calculator");
			},
			_ => panic!("There are some error"),
		};
	};
}


fn int_from_input(mut str: String) -> i64 {
	loop {
		io::stdin()
		.read_line(&mut str)
		.expect("Failed to read line");

		let _some_number = match str.trim().parse() {
				Ok(_some_number) => {
					break _some_number
				}
				Err(_) => continue,
		};
	}
}

fn calc(mut a: i64, b: i64, todo: String) -> i64 {
	match todo.as_str().trim() {
		"+" => a + b,
		"-" => a - b,
		"*" => a * b,
		"/" => a / b,
		"**" => {
			for _b in 0..b {
				a = a * a;
			};
			a
		},
		"%" => a % b,
		"0" => panic!("You exit the rust calculator"),
		_ => {
			println!("Invalid function");
			0
		},
	}
}