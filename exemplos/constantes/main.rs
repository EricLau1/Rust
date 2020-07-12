fn main() {
    // Constante são imutáveis
    const UP:char = 'w';
    println!("Constante: {}", UP);

    const PI:f64 = 3.14159;
    println!("Constante: {}", PI);

    const AVAILABLE:bool = true;
    println!("Constante: {}", AVAILABLE);

    // const mut PI:f64 = 3.14159; ERROR
    // println!("Constante: {}", PI);
}