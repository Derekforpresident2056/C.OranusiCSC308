#[derive(Debug) ]

struct Circle {
	radius: f32,	
}

impl Circle {

	fn area(&self) -> f32 {
		3.142 * (self.radius * self.radius)
	}

	fn circumference(&self) -> f32 {
		2.0 * 3.142 * self.radius
	}
}

fn main() {
	let circle1 = Circle {
		radius: 40.0,
	};

	println!("Circle radius is { :? }", circle1);
	println!("Area of circle is { :? }",circle1.area());
	println!("circumference of circle is { :? }",circle1.circumference());
}
}
