use std::io;

fn main() {

    let mut input = String::new();

    println!("Digite seu nome: ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Seu nome é: {}", input);
        },
        Err(e) => println!("Oops! Ocorreu um erro: {}", e)
    }

    input = String::new();
    println!("Digite um número: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler da entrada padrão!");

    let trimmed = input.trim();
    match trimmed.parse::<u32>() {
        Ok(n) => {
            println!("{} * {} é igual a {}", n, n, n*n);
        },
        Err(e) => println!("Erro ao tentar converter valor {}: Error: {}", input, e)
    }
}