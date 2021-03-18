use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    caluculation: T,
    // value: Option<u32>,
    value: Option<HashMap<String, u32>>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(caluculation: T) -> Cacher<T> {
        Cacher {
            caluculation,
            value: None,
        }
    }

    fn value(&mut self, args: u32) -> u32 {
        match &mut self.value {
            Some(v) => {
                v.entry(args.to_string()).or_insert(args);
                args
            }
            None => {
                let v = (self.caluculation)(args);
                let mut hash_map = HashMap::new();
                hash_map.insert(args.to_string(), args);
                self.value = Some(hash_map);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("caluculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "今日は{}回腕立て伏せをしてください！",
            expensive_result.value(intensity)
        );

        println!(
            "次に{}回腹筋をしてください!!!!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("今日は休憩してください！水分休憩を忘れずに");
        } else {
            println!(
                "今日は{}分間走ってください!!!!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
