fn main() {
    let s = String::from("hello");
    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let s = String::from("hello, world");
    let f_s = first_word(&s);
    println!("firtst word: {}", f_s);

    let f_s2 = first_word2(&s[..]);
    println!("firtst word2: {}", f_s2);
}

// リストの参照を返す。これであれば可能。
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// strは文字列スライス型
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
