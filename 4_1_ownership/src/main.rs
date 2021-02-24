fn main() {
  println!("Hello, world!");
  let mut s = String::from("hello"); //ここからsは有効になる
  s.push_str(", world!"); // 有効
  println!("{}", s); // 有効

  // 整数型は参照後の古い変数へのアクセスも可能
  let x = 5;
  let y = x;
  println!("x={}, y={}", x, y);
  // String型はヒープにもデータが保持されるので、古い変数へのアクセスが不可能
  let s1 = String::from("hello");
  // let s2 = s1; //この操作はs1のstackをs2に"move"させる
  // println!("{}, World", s1); するとこれはmoveした後の変数を指定しているので動かない
  let s2 = s1.clone();
  println!("s1={}, s2={}", s1, s2);

  let s = String::from("hello"); // sがスコープへ入る
  takes_ownership(s); //sが関数にmoveされ、有効でなくなる
                      // println!("{}", s); //やはりこれはだめ

  let x = 5;
  makes_copy(x); // xが関数にmoveされるが、i32はcopyなのでこの後も生きている
  println!("{}", x); //これはOK
}

fn takes_ownership(some_string: String) {
  println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
  println!("{}", some_integer);
}
