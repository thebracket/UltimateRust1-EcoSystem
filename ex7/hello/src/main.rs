use greeter::greet_user;

fn main() {
    let result = greet_user();
    if let Ok(user_name) = result {
        println!("Hello, {user_name}");
    } else {
        println!("{:?}", result);
    }
}
