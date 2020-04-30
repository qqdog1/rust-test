fn main() {
    let circle = Circle{ radius: 5};
    let rectangle = Rectangle{ height: 5, width: 10};

    let shape1 = Shape::CCC(circle);
    let shape2 = Shape::RRR(rectangle);

    println!("{}", get_area(shape1));
    println!("{}", get_area(shape2));
}

fn get_area(shape: Shape) -> f64 {
    match shape {
        Shape::CCC(c) => c.area(),
        Shape::RRR(r) => r.area(),
    }
}

enum Shape {
    CCC(Circle),
    RRR(Rectangle),
}

struct Rectangle {
    width: u16,
    height: u16,
}

impl Rectangle {
    fn area(&self) -> f64 {
        (self.width * self.height) as f64
    }
}

struct Circle {
    radius: u16,
}

impl Circle {
    fn area(&self) -> f64 {
        self.radius as f64 * 3.14
    }
}