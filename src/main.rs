fn main() {
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(5.0, 10.0);
    let square = Shape::Square(5.0);
    println!("Area of circle {}", circle.area());
    println!("Area of rectangle {}", rectangle.area());
    println!("Area of square {}", square.area());
}

enum Shape {
    Square(f32),
    Rectangle(f32, f32),
    Circle(f32),
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Square(side) => side * side,
            Shape::Rectangle(length, width) => length * width,
            Shape::Circle(radius) => std::f32::consts::PI * (radius * radius),
        }
    }
}
