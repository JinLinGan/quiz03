// 定义TrafficLight枚举
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

// 定义Trait
trait GetDuration {
    fn get_duration(&self) -> u32;
}

// 实现Trait
impl GetDuration for TrafficLight {
    fn get_duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 1,
            TrafficLight::Green => 10,
            TrafficLight::Yellow => 100,
        }
    }
}

fn main() {
    println!(
        "Traffic Light {} duration is {}",
        "Red",
        TrafficLight::Red.get_duration()
    );
    println!(
        "Traffic Light {} duration is {}",
        "Green",
        TrafficLight::Green.get_duration()
    );
    println!(
        "Traffic Light {} duration is {}",
        "Yellow",
        TrafficLight::Yellow.get_duration()
    );
}
