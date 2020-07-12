fn main() {
    // Imutável por padão
    let x = 10;
    println!("Valor Imutável: {}", x);

    // x = 15; ERROR!

    // Variável mutável
    let mut y = 1;
    println!("Valor Mutável: {}", y);
    y = 10;
    println!("Valor Mutável: {}", y);
 
    // shadowing 
    // redeclarando uma variável
    let mut x = 20;
    println!("Shadowing: {}", x);
    x += 1;
    println!("Shadowing: {}", x);
}