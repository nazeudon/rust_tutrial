fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello"); //ここからsは有効になる
    s.push_str(", world!"); // 有効
    println!("{}", s); // 有効
} // ここでスコープを抜ける
  // もう有効では無い
