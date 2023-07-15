pub fn calc(mut a: f64, b: f64, todo: &str) -> f64 {
	match todo {
		"+" => a + b,
		"-" => a - b,
		"*" => a * b,
		"/" => a / b,
		"**" => {
			let num = a;
			for _i in 1..unsafe { b.to_int_unchecked() } {
				a = a * num;
			};
			a
		},
		"%" => a % b,
		_ => {
			println!("Function doesn't exist. Please try again.");
			return 0.000000;
		},
	}
}