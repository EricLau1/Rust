fn main() {
    let x = 10;
    let result = {
        let y = 100;
        y * x
    };

    println!("Resultado da expressão: {}", result); // 1000
}