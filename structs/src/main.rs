struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        email: "someone@example.com".to_string(),
        username: "someusername123".to_string(),
        active: true,
        sign_in_count: 1
    };
    println!("{}", user1.email);
}
