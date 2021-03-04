fn main() {
    println!("Hello, world!");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
    println!("{}", s3);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for c in "あいうえお".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    println!("{}", &s3[0..1]);
}
