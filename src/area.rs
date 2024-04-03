pub trait Area {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.1415926 * self.radius * self.radius
    }
}

pub struct Square {
    pub side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub fn calc_area<T: Area>(shape: T) {
    println!("area is: {}", shape.area());
}
