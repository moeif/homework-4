mod area;
mod sumu32;
mod traffic_light;
use area::*;
use sumu32::*;
use traffic_light::*;

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    println!("red time: {}", red.time());
    println!("yellow time: {}", yellow.time());
    println!("green time: {}", green.time());

    let numbers = vec![100, 10000, 100000, 10000];
    println!("sum is: {:?}", sum_u32(&numbers));

    let numbers = vec![u32::MAX, u32::MAX, 124];
    println!("sum is: {:?}", sum_u32(&numbers));

    let circle = Circle { radius: 10.0 };
    let square = Square { side: 5.0 };
    calc_area(circle);
    calc_area(square);
}
