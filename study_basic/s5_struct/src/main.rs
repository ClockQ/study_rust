#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("email1");
    println!("user1 = {:#?}", user1);

    let user2 = build_user(
        String::from("email2"), 
        String::from("username2")
    );
    println!("user2 = {:#?}", user2);

    let user3 = User {
        email: String::from("email3"),
        ..user1
    };
    println!("user3 = {:#?}, {} {} {} {}", user3, user3.username, user3.email, user3.active, user3.sign_in_count);
}
