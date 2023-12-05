fn main() {
    let saludo = String::from("Hola");

    let closure = move || {
        println!("La closure dice: {}", saludo);
    };

    // Intenta usar la variable 'saludo' después de haber sido movida a la closure
    println!("Fuera de la closure: {}", saludo);

    // Ejecuta la closure
    closure();
}
