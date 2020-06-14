mod metodes;
mod poligons;
mod users;

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

    let num: i32 = 3;
    match num {
        1 => println!("Numero uno"),
        2 => println!("Numero dos"),
        3 => println!("Numero tres"),
        _ => println!("Numero default {}", num),
    }

    let num: i32 = 5;

    match num {
        1 => println!("Numero uno"),
        2 => println!("Numero dos"),
        3 => println!("Numero tres"),
        4 | 5 | 6 => println!("Entre 4 i 6"),
        7..=100 => {
            println!("Es major o igual a 7");
            println!("Aixo es un rang")
        }
        _ => println!("Numero {}", num),
    }

    let message = match num {
        1 => "Numero uno",
        2 => "Numero dos",
        3 => "Numero tres",
        _ => "Numero",
    };

    println!("Result: {}", message);

    for numero in 1..31 {
        match (numero % 3, numero % 5) {
            (0, 0) => println!("Fizz buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", numero),
        }
    }

    println!("-----------------");

    // Els enum sempre en CamelCase
    enum Response {
        Success,
        Error(u32), // 403, 404, 500
    }

    let respuesta = Response::Success;
    match respuesta {
        Response::Success => println!("La peticio ha anat ok"),
        Response::Error(403) => println!("Forbidden"),
        Response::Error(404) => println!("Not foound"),
        Response::Error(_) => println!("Error identificat"),
    }
    let respuesta = Response::Error(403);

    match respuesta {
        Response::Success => println!("La peticio ha anat ok"),
        Response::Error(403) => println!("Forbidden"),
        Response::Error(404) => println!("Not foound"),
        Response::Error(_) => println!("Error identificat"),
    }

    enum ResponseComplete {
        Error(u32, String), // 403, 404, 500
    }

    let respuesta = ResponseComplete::Error(405, String::from("Problema..."));

    match respuesta {
        ResponseComplete::Error(403, _) => println!("Forbidden"),
        ResponseComplete::Error(404, _) => println!("Not foound"),
        ResponseComplete::Error(_, mensage) => println!("Error: {}", mensage),
    }
    println!("-----------------");

    let result = metodes::suma(1, 2);
    println!("Resultat: {}", result);
    println!("-----------------");

    //str -> Stack
    //String -> Heap

    let var_str = "Hola";
    let mut var_string = String::from("Hola");
    var_string.push(' ');
    var_string.push('a');
    var_string.push_str("Holaaaaa");
    println!("Resultat: {}", var_str);
    println!("Resultat: {}", var_string);
    println!("-----------------");

    let rect = poligons::Rectangulo { alto: 10, ancho: 5 };
    let res = poligons::area(rect);
    println!("Resultat {}", res);

    //  Com s'ha usat rect en la funcio area ha canviat d'owner.
    // * Cada valor en rust te el seu propi ownership
    // * Nomes pot existir un ownership a la vegada
    // * Si un ownership surt del seu alcanç es descartara(finalitza la funcio)
    let rect = poligons::Rectangulo { alto: 10, ancho: 5 };
    println!("Alto {} - Ancho {}", rect.alto, rect.ancho);
    println!("-----------------");

    // Mostra d'ownership amb "pointers"
    let rect1 = poligons::Rectangulo { alto: 10, ancho: 5 };
    let res = poligons::area_pointer(&rect1);
    println!("Alto {} ", res);
    println!("Alto {} - Ancho {}", rect1.alto, rect1.ancho);
    println!("-----------------");

    let mut user = users::User {
        username: String::from("Usuario"),
        password: String::from("Password"),
    };

    user.saluda();
    user.change_password(String::from("abc"));
    println!("New pass: {}", user.password);
    println!("-----------------");

    #[derive(Debug)]
    struct Color(u32, u32, u32);
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    println!("El color es: {:?}", white);
    println!("El color es: {:?}", black);

    println!("-----------------");

    let message = String::from("Hola Josep");
    let hola = &message[0..4];
    println!("El mes: {}", hola);
}
