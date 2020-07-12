fn main() {

    let x = 10;
    let y = x; // cópia rasa 
    
    println!("X: {}, Y:{}", x, y);

    let text = String::from("Hello");
    let message = text.clone(); // cópia profunda

    println!("Text: {}, Message: {}", text, message);
}