fn main() {
	// define constant variable:
	let lang = "Rust";

	// print string with variable:
	println!("Hello, {}!", lang);

	// define type:
	type MyType = int;

	use_if_else();

	use_match();

	use_while();

	use_loop();

	use_for_in();

	use_struct();

	//use_enum();
}

// define function:
fn is_ten(x: int) -> bool {
	// No need for a return statement.
	// The result of the expression is used as the return value. 
	x == 10
}

fn use_if_else() {
	// define variable with if-else expression:
	let x =
		if is_ten(10) {
			1i
		} else {
			0i
		};
	println!("x = {}", x);
}

fn use_match() {
	let month = 7i;
	// like 'switch/case' in other languages.
	// like golang, there's no 'falling through'.
	let max_day =
		match month {
			2 => 28i,
			4 | 6 | 9 | 11 => 30i,
			_ => 31i
		};
	println!("The max day of month({}) is {};", month, max_day);
}

fn use_while() {
	// define mutable variable: 
	let mut count = 0i; // 'count' is an 'int'
	while count < 10 {
		println!("count: {}", count);
		count += 1;// '++' operation is not supported.
	}
}

fn use_loop() {
	let mut x = 100u;
	//like 'while(true)'
	loop {
		x -= 10;
		println!("x: {}", x);
		if x <= 0 {
			break;
		}
	}
}

fn use_for_in() {
	for y in range(0u, 10) {
		println!("y: {}", y);
	}
	let str = "RincLiu";
	for chr in str.chars() {
		println!("{}", chr);
	}
}

//Define struct type:
struct Location {
	longitude: f64,
	latitude: f64
}

fn use_struct() {
	let mut myLocation = Location { longitude: 121.3581038898, latitude: 31.2198830738 };
	myLocation.latitude += 1.0;
	println!("My location: {}, {}", myLocation.longitude, myLocation.latitude );
}

//Define Enum type:
//Note: This feature of the compiler is currently gated behind the #[feature(struct_variant)] directive.
/*
enum Shape {
	Circle { center: Point, radius: f64 },
	Retangle { top_left: Point, bottom_right: Point }
}

struct Point {
	x: f64,
	y: f64
}

fn area(shape: Shape) -> f64 {
	match shape {
		Circle { radius: radius, .. } => f64::consts::PI * square(radius),
		Retangle { top_left: top_left, bottom_right: bottom_right } => {
			(bottom_right.x - top_left.x) * (top_left.y - bottom_right.y)
		}
	}
}

fn use_enum() {
	let c = Point { x: 0, y: 0};
	let r = 12345.67890;
	let circie = Circle { center: c, radius: r};
	println!("Circle's area: {}", area(circle));
}
*/
