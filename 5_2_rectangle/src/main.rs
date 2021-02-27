// fn main() {
//     let width = 30;
//     let height = 50;

//     println!(
//         "The area of the rectangle is {} aquare pixels.",
//         area(width, height)
//     )
// }
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} aquare pixels.",
//         area(rect1)
//     )
// }
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} aquare pixels.",
        area(&rect1)
    );
    println!("rectangle is {:?}", rect1);
    println!("rectangle is {:#?}", rect1);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
