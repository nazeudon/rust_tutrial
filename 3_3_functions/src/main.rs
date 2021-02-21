fn main() {
    println!("Hello, world!");
    another_functinos(5);
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn another_functinos(x: i32) {
    println!("Another functions.");
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
