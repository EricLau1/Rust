fn main() {
    say_hello("John");
    sum(10, 2);

    let pi = get_pi();
    println!("{}", pi);

    let msg = get_msg();
    println!("{}", msg);
}

fn sum(x:i32,y:i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

fn get_pi() -> f64 {
    3.14159
}

fn get_msg() -> &'static str {
    "Hello World!"
}