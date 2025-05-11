// trait Shape {
//     fn area(&self) -> f64;
// }

// struct Circle {
//     radius: f64,
// }

// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// impl Shape for Circle {
//     fn area(&self) -> f64 {
//         3.14 * self.radius * self.radius
//     }
// }

// impl Shape for Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
// }

// fn print_area(shape: &dyn Shape) {
//     println!("面积: {}", shape.area());
// }

// fn main() {
//     let circle = Circle { radius: 10.0 };
//     let rectangle = Rectangle {
//         width: 10.0,
//         height: 20.0,
//     };

//     print_area(&circle);
//     print_area(&rectangle);
// }

//use shapes::Shape;

fn main() {
    let circle: shapes::Circle = shapes::Circle::from(13.5);
    let rectangle: shapes::Rectangle = shapes::Rectangle::from(15.0, 14.0);
    shapes::print_area(&circle);
    shapes::print_area(&rectangle);

    let circle: shapes::Circle = shapes::Circle::new()
        .set_radius(13.63);
    let rectangle: shapes::Rectangle = shapes::Rectangle::new()
        .set_width(12.321)
        .set_height(312.123);

    shapes::print_area(&circle);
    shapes::print_area(&rectangle);

    shapes::print_perimeter(&circle);
    shapes::print_perimeter(&rectangle);
}

mod shapes{
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

    pub fn print_area(shape: &dyn Shape) {
        println!("面积: {:.2}", shape.area());
    }

    pub fn print_perimeter(shape: &dyn Shape) {
        println!("周长: {:.2}", shape.perimeter());
    }
}