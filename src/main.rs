use rust_calculator::functional::menu::introduction::introduction;
use rust_calculator::functional::methods::equation;

fn main() {
	introduction();
	
	let mut history: Vec<String> = Vec::new();
	let mut history_results: Vec<f64> = Vec::new();

	loop {
		let equation_for_history = equation::input_equation(&history, &history_results);
		
		Vec::push(&mut history, equation_for_history.0);
		Vec::push(&mut history_results, equation_for_history.1);

		if equation_for_history.2 == true {
			history.clear();
			history_results.clear();
			println!("History cleared!\n");
		}
	}
}
