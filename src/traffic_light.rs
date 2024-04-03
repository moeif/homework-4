pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub trait TrafficLightTime {
    fn time(&self) -> u32;
}

impl TrafficLightTime for TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 40,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }
}
