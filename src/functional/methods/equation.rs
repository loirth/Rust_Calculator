use std::io;

use crate::functional::methods::converters::from_input;
use crate::functional::menu::{exit, history};
use crate::math::basic_operators;

																	//equation, res, clear history 
pub fn input_equation(history: &Vec<String>, result_history: &Vec<f64>) -> (String, f64, bool) {
	let mut result_from_history = (false, false);
	
	loop {
		println!("                    What you wanna do?");
		println!("Exit  |  History  |  +  |  -  |  *  |  /  |  **  |  %  ");


		let mut some_do: String = String::new();

		io::stdin()
			.read_line(&mut some_do)
			.expect("Failed to read line");

		some_do = some_do.as_str().to_lowercase();
		let some_do: &str = some_do.as_str().trim();

		match some_do {
			"exit" => exit::exit(),
			"history" => {
				result_from_history = history::manage_history(history, result_history);
				match result_from_history.0 {
	    			true => return (String::new(), 0.0, true),
	    			false => continue,
				};
			},
			"+" => (),
			"-" => (),
			"*" => (),
			"/" => (),
			"**" => (),
			"%" => (),
			_ => {
				println!("Invalid command! Please try again!\n");
				continue;
			},
		};
		
		let first_number = match result_from_history.1 {
			true => match get_last(result_history) {
				Some(n) => {
					println!("First number is: {}", n);
					*n
				},
				None => {
					println!("There is no last element in history\n");
					result_from_history.1 = false;
					continue;
				},
			},
			false => from_input::float_from_input(String::from("first")),
		};
		

		let second_number = from_input::float_from_input(String::from("second"));

		let result: f64 = basic_operators::calc(first_number, second_number, some_do);

		println!("The result is: {}\n", result);


		let equation = format!("{} {} {} = {}", first_number, some_do, second_number, result);

		return (equation, result, false);
	}
}

fn get_last(vec: &Vec<f64>) -> Option<&f64> {
	match vec.len() {
		0 => vec.get(0),
		n => vec.get(n - 1),
	}
}