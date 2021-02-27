fn main() {
    let user1 = User {
        email: String::from("nazeudon@example.com"),
        username: String::from("nazeudon"),
        sign_in_count: 1,
        active: true,
    };
    let user3 = User {
        email: String::from("takeshi@example.com"),
        username: String::from("takeshi"),
        ..user1 //明示されていない箇所はuser1から引き継ぐ
    };
    let user2_username = String::from("maccho");
    let user2_email = String::from("maccho@email.com");
    let user2 = builder_user(user2_email, user2_username);

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!("{}", user1.active);

    println!("{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.sign_in_count);
    println!("{}", user2.active);

    println!("{}", user3.username);
    println!("{}", user3.email);
    println!("{}", user3.sign_in_count);
    println!("{}", user3.active);

    println!("{}, {}, {}", black.0, black.1, black.2);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn builder_user(email: String, username: String) -> User {
    // Userを返すことも可能
    User {
        email,
        username,
        active: false,
        sign_in_count: 2,
    }
}
