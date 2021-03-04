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

    let v4 = vec![1, 2, 3, 4];
    let third: &i32 = &v4[2];
    let third2: Option<&i32> = v.get(2);
    println!("The value of v4: {:?}", v4);
    println!("The value of third: {:?}", third);
    println!("The value of third2: {:?}", third2);

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{}", i);
    }
    println!("The value of v5: {:?}", v5);

    let mut v6 = vec![100, 32, 54];
    for i in &mut v6 {
        *i += 50;
    }
    println!("The value of v6: {:?}", v6);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
    println!("The value of row: {:?}", row);
}
