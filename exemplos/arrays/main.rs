fn main() {
    println!("Arrays");

    // array do tipo float com no maximo quatro elementos
    let array1: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
    println!("Array 1: size: {}, index[{}] -> {}", array1.len(), 0, array1[0]);

    let array2: [i8; 3] = [0;3];
    for i in array2.iter() {
        println!("{}", i);
    }   
    
    println!("Tuplas");

    let tupla: (f32, i8, String) = (0.5, 1, "Hello".to_string());
  
    let (x, y, z) = tupla;
  
    println!("o valor do x: {}", x);
    println!("o valor do y: {}", y);
    println!("o valor do z: {}", z);
}
