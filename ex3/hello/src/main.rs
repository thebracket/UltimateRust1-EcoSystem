mod greeter;
use greeter::greet_user;

fn main() {
    let user_name = greet_user();
    println!("Hello, {user_name}");
}
