use std::io;

use crate::functional::methods::converters::float_from_input::float_from_input;
use crate::functional::methods::exit::exit;
use crate::math::basic_operators::calc;


pub fn equation(history: &Vec<String>, _history_results: &Vec<f64>) -> (String, f64) {
	loop {
	println!("                  What you wanna do?");
	println!("Exit  |  History  |  +  |  -  |  *  |  /  |  **  |  %  ");

	let mut some_do: String = String::new();

	io::stdin()
		.read_line(&mut some_do)
		.expect("Failed to read line");

	some_do = some_do.as_str().to_lowercase();
	let some_do: &str = some_do.as_str().trim();

	match some_do {
		"exit" => exit(),
		"history" => {
			println!("{:#?}", history);
			continue;
		},
		"+" => (),
		"-" => (),
		"*" => (),
		"/" => (),
		"**" => (),
		"%" => (),
		_ => continue,
	}

	let first_number = float_from_input(String::from("first"));

	let second_number = float_from_input(String::from("second"));

	let result: f64 = calc(first_number, second_number, some_do);

	println!("The result is: {}", result);


	let equation = format!("{} {} {} = {}", first_number, some_do, second_number, result);

	return (equation, result);
	}
}