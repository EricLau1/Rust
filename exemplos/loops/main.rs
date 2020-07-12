fn main() {
    // forever_repeat("Hello World!");

    let mut i = 0;

    println!("Iniciando loop...");
    loop {
        println!("{}", i);
        if i == 3 {
            
            break
        }
        i += 1;
    }
    println!("Fim do loop.");

    println!("Iniciando while...");
    while i != 0 {
        println!("{}", i);
        i -= 1;
    }
    println!("Fim do while.");

    println!("Iniciando o for em ordem crescente...");
    for n in 0..4 {
        println!("O numero é: {}", n);
    }
    println!("Fim do for.");

    println!("Iniciando o for em ordem decrescente...");
    for n in (0..4).rev() {
        println!("O numero é: {}", n);
    }
    println!("Fim do for.");
}

#[allow(dead_code)]
fn forever_repeat(message: &str) {
    loop {
        println!("{}", message);
    }
}