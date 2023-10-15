// Definición de un trait llamado Hablable
trait Hablable {
    fn hablar(&self);
}

// Implementación del trait para diferentes tipos
struct Perro;
struct Gato;

impl Hablable for Perro {
    fn hablar(&self) {
        println!("¡Guau!");
    }
}

impl Hablable for Gato {
    fn hablar(&self) {
        println!("¡Miau!");
    }
}

// Función genérica que saluda a cualquier valor que implemente el trait Hablable
fn saludo(sujeto: &dyn Hablable) {
    sujeto.hablar();
}

fn main() {
    let perro = Perro;
    let gato = Gato;

    // Llamada a la función saludo con diferentes tipos
    saludo(&perro); // Salida: ¡Guau!
    saludo(&gato);  // Salida: ¡Miau!
}
