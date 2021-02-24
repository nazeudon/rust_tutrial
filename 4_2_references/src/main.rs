// ①-1, 参照を用いない場合の書き方.少し冗長.
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len)
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// ①-2, 参照を用いた書き方.スッキリ.
// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// ②-1参照した値を変更することはできない.
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// これであれば可能.sを可変とする
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);

    // ダメ！.参照は1度までしかできない！！
    // let r1 = &mut s;
    // let r2 = &mut s;

    // 参照を返すことはできない
    // let reference_to_nothing = dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     //dangleはStringへの参照を返す
//     let s = String::from("hello"); //sは新しいString

//     &s //String sへの参照を返す
// } //ここでsはスコープを抜け、ドロップされる。そのメモリは消される
