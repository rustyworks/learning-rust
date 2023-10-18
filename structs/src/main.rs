fn main() {
    let user_1 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 64,
    };

    let mut user_2 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        sign_in_count: 64,
    };
    user_2.email = String::from("another@example.com");

    let user_3 = User {
        email: String::from("kentang@example.com"),
        ..user_1
    };

    println!("user_3: {0}, {1}, {2}, {3}", user_3.username, user_3.email, user_3.sign_in_count, user_3.active);
    println!("user_1: {0}, {1}, {2}, {3}", user_1.username, user_1.email, user_1.sign_in_count, user_1.active);

    let user_4 = build_user(String::from("asd@asd.com"), String::from("gaga"));
    println!("{}", user_4.username);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
