use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 20); //上書き
    println!("{:?}", scores);
    let team_name = String::from("Blue");
    let none_name = String::from("Red");
    let score = scores.get(&team_name);
    let score_none = scores.get(&none_name);
    println!("{:?}", score); // Optionを返す
    println!("{:?}", score_none); // Optionを返す

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name, field_valueはmoveされているので以下では参照できない

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores3);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    // これはおもろい書き方
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
