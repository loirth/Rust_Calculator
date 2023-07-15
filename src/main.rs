use rust_calculator::functional::menu::introduction::introduction;
use rust_calculator::functional::methods::input_equation;

fn main() {
	introduction();
	
	let mut history: Vec<String> = Vec::new();
	let mut history_results: Vec<f64> = Vec::new();

	loop {
		let equation_for_history = input_equation::equation(&history, &history_results);
		
		Vec::push(&mut history, equation_for_history.0);
		Vec::push(&mut history_results, equation_for_history.1);
	}
}
