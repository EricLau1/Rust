fn main() {

    println!("Tipos de Variáveis");

    let base:u64 = 2;

    println!("Inteiros (Signed)");

    let int_8bits:i8 = ((base.pow(8) / 2) - 1) as i8;
    println!("Um inteiro de 8 bits: i8 -> {}", int_8bits);

    let int_16bits:i16 = ((base.pow(16) / 2) - 1) as i16;  
    println!("Um inteiro de 16 bits: i16 -> {}", int_16bits);

    let int_32bits:i32 = ((base.pow(32) / 2) - 1) as i32;  
    println!("Um inteiro de 32 bits: 32 -> {}", int_32bits);

    let int_64bits:i64 = ((u64::MAX / 2) - 1) as i64;  
    println!("Um inteiro de 64 bits: i64 -> {}", int_64bits);

    println!("Inteiros (Unsigned)");

    let uint_8bits:u8 = u8::MAX;
    println!("Um inteiro unsigned de 8 bits: u8 -> {}", uint_8bits);

    let uint_16bits:u16 = u16::MAX;  
    println!("Um inteiro unsigned de 16 bits: u16 -> {}", uint_16bits);

    let uint_32bits:u32 = u32::MAX;  
    println!("Um inteiro de 32 bits: u32 -> {}", uint_32bits);

    let uint_64bits:u64 = u64::MAX;  
    println!("Um inteiro de 64 bits: u64 -> {}", uint_64bits);

    println!("Ponto Flutuante");
    
    let float_32bits:f32 = 1.0 / 2.0;
    println!("Um float de 32 bits: f32 -> {}", float_32bits);

    let float_64bits:f64 = 3.14159;
    println!("Um float de 64 bits: f64 -> {}", float_64bits);

    println!("Booleanos");

    let booleano:bool = true;
    println!("Um booleano verdadeiro -> {}", booleano);

    println!("Um booleano falso -> {}", !booleano);

    println!("Caracteres");

    let caractere:char = 'A';
    println!("Um caractere: char -> {}", caractere);

    println!("String");

    let message:String = String::from("Hello World!");
    println!("Uma String: message1 -> {}", message);

    let message2:String = "Olá Mundo!".to_string();
    println!("Uma String: message2 -> {}", message2);
}
