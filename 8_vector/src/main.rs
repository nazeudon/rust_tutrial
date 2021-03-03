fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();
    println!("The value of v: {:?}", v);
    let v2 = vec![1, 2, 3];
    println!("The value of v2: {:?}", v2);

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    println!("The value of v3: {:?}", v3);
}
