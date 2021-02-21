fn main() {
    /* variable */
    let mut x = 5;
    println!("The variable of x is: {}", x);
    x = 6;
    println!("The variable of x is: {}", x);

    const MAX_OPTIONS: u32 = 100_000;
    println!("MAX_OPTIONS : {}", MAX_OPTIONS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces: {}", spaces);

    /* type */
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{}, {}", first, second)
}
