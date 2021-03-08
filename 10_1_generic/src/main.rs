fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number
    //     }
    // }
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'm', 'b', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x())
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
