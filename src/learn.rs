fn main() {
    let circle: shapes::Circle = shapes::Circle::from(13.5);
    let rectangle: shapes::Rectangle = shapes::Rectangle::from(15.0, 14.0);
    shapes::print_area("Circle13.5",&circle);
    shapes::print_area("Rectangle15.0,14.0",&rectangle);

    let circle: shapes::Circle = shapes::Circle::new()
        .set_radius(13.63);
    let rectangle: shapes::Rectangle = shapes::Rectangle::new()
        .set_width(12.321)
        .set_height(312.123);

    shapes::print_area("Circle13.63",&circle);
    shapes::print_area("Rectangle12.321,312.123",&rectangle);

    shapes::print_perimeter("Circle13.63",&circle);
    shapes::print_perimeter("Rectangle12.321,312.123",&rectangle);

    let triangle: shapes::EquilateralTrangle = shapes::EquilateralTrangle::new()
        .set_base(1.12);
    let triangle1: shapes::EquilateralTrangle = shapes::EquilateralTrangle::from(12.43);
    shapes::print_area("Triangle1.12",&triangle);
    shapes::print_perimeter("Triangle1.12",&triangle);

    shapes::print_area("Triangle12.43",&triangle1);
    shapes::print_perimeter("Triangle12.43",&triangle1);
}

mod shapes{
    use core::str;
    use std::f64::consts::PI;
    pub trait Shape {
        fn area(&self) -> f64;
        fn perimeter(&self) -> f64;
    }

    pub struct Circle {
        radius: f64,
    }

    pub struct Rectangle {
        width: f64,
        height: f64,
    }
    
    pub struct EquilateralTrangle {
        base: f64,
    }

    impl EquilateralTrangle {
        pub fn new() -> Self {
            EquilateralTrangle { base: 0.0 }
        }

        pub fn from(base: f64) -> Self {
            EquilateralTrangle { base }
        }

        pub fn set_base(mut self, base: f64) -> Self {
            self.base = base;
            self
        }
    }

    impl Shape for EquilateralTrangle {
        fn area(&self) -> f64 {
            0.5 * self.base * self.base * ((60.0 * PI) / 180.0).sin()
        }

        fn perimeter(&self) -> f64 {
            // 这里假设是等边三角形
            3.0 * self.base
        }
    }

    impl Shape for Circle {
        fn area(&self) -> f64 {
            3.14 * self.radius * self.radius
        }

        fn perimeter(&self) -> f64 {
            2.0 * 3.14 * self.radius
        }
    }

    impl Circle {
        pub fn new() -> Self {
            Circle { radius: 0.0 }
        }

        pub fn from(radius: f64) -> Self {
            Circle { radius }
        }

        pub fn set_radius(mut self, radius: f64) -> Self {
            self.radius = radius;
            self
        }
    }

    impl Shape for Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }
    }

    impl Rectangle {
        pub fn new() -> Self {
            Rectangle { width: 0.0, height: 0.0 }
        }

        pub fn from(width: f64, height: f64) -> Self {
            Rectangle { width, height }
        }

        pub fn set_width(mut self, width: f64) -> Self {
            self.width = width;
            self
        }

        pub fn set_height(mut self, height: f64) -> Self {
            self.height = height;
            self
        }
    }

    pub fn print_area(tip:&str,shape: &dyn Shape) {
        println!("{}面积: {:.2}",tip, shape.area());
    }

    pub fn print_perimeter(tip:&str,shape: &dyn Shape) {
        println!("{}周长: {:.2}", tip,shape.perimeter());
    }
}