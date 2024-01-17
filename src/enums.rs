pub enum Shape {
    Circle(f64),
    Square(f64),
}

pub fn total_area(shapes: &[Shape]) -> f64 {
    let area = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side) => side * side,
        })
        .sum();

    println!("total_area: {} square units", area);
    area
}
