fn main() {
    println!("Hello, world!");
    let numbers = [1, 2, 4, 5];
    println!("El valor es: {:?}", numbers);

    let mut vector = vec![1, 2, 3];
    vector.push(4);
    vector.push(5);
    vector.insert(0, -1);
    vector.insert(1, 0);
    vector.remove(vector.len() - 1);
    vector[0] = -10;
    let primer_element = vector[0];
    let ultim_element = vector.pop().unwrap();
    println!("vector: {:?}", vector);
    println!("{}", primer_element);
    println!("{}", ultim_element);

    let numeros: [i32; 5] = [1, 2, 3, 4, 5];

    for numero in numeros.iter() {
        println!("num: {}", numero);
    }

    for numero in 1..10 {
        println!("num: {}", numero);
    }
}
