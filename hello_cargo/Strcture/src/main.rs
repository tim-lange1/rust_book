fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("someone.ext@studing.org"),
        username: String::from("someone1"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user2
    };
    user1.email = String::from("anotheremail@example.com");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}