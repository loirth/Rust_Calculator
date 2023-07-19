use crate::functional::menu::exit::exit;

pub fn what_to_do() {
	use std::io;

	loop {
		println!("        What you wanna do?       ");
		println!("     Start  |  Help  |  Exit"       );

		let mut start_command: String = String::new();
	
		io::stdin()
			.read_line(&mut start_command)
			.expect("Failed to read line");
		
		match start_command.as_str().to_lowercase().as_str().trim() {
			"start" => {
				break;
			},
			"help" => {
				break;
			},
			"exit" => {
				exit();
			},
			_ => {
				println!("Invalid command! Please try again!\n");
				continue;
			},
		};
	};
}