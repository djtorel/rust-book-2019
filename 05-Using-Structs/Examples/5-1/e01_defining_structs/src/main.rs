
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1.email is: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("user1.email is now: {}", user1.email);

    let user2 = build_user(String::from("foo@bar.com"), String::from("FooBar"));

    println!("user2.username: {}", user2.username);

    let user2 = User {
        email: String::from("anotherfoo@anotherbar.com"),
        username: String::from("AnotherFooBar"),
        ..user1
    };

    println!("Now user2.username is {}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
