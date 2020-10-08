use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("rechnet langsam...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {

    let mut expensiv_result = Cacher::new(|num| {
        println!("rechnet langsam...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Mach heute {} Liegestütze!", expensiv_result.value(intensity));
        println!("Als nächstes {} Sit-ups!", expensiv_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Mach heute eine Pause! Denk daran, ausreichend zu trinken!");
        } else {
            println!("Heute, {} Minuten Lauftrainig!",expensiv_result.value(intensity));
        }
    }
}

struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}


impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}