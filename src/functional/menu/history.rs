use std::io;

use crate::functional::menu::exit;
												  //clear history or not, take last value from results or not
pub fn manage_history(history: &Vec<String>, results: &Vec<f64>) -> (bool, bool) {
	print_history(history);

	loop{
		println!("               What you wanna do next?");
		println!("History  |  Continue  |  Clear  |  Take  |  Results  |  Exit");

		let mut some_do: String = String::new();

		io::stdin()
			.read_line(&mut some_do)
			.expect("Failed to read line");

		match some_do.as_str().to_lowercase().as_str().trim() {
			//"help" => help::help(),
			"history" => {
				print_history(history);
				continue;
			},
			"continue" => return (false, false),
			"clear" => return (true, false),
			"take" => return (false, true),
			"results" => {
				print_results(results);
				continue;
			}
			"exit" => exit::exit(),
			_ => {
				println!("Invalid command! Please try again!\n");
				continue;
			},
		};
	};
}


fn print_history(history: &Vec<String>) {
	println!("      History");
	println!("{:#?}\n", history);
}


fn print_results(results: &Vec<f64>) {
	println!("   Results");
	println!("{:#?}\n", results);
}