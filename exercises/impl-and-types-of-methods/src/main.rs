// Definición de un trait llamado MiTrait
trait MiTrait {
    // Un método de trait que toma una referencia a self y devuelve un valor u32
    fn metodo_de_trait(&self) -> u32;
}

#[derive(Debug)]
// Definición de una estructura llamada MiTipo
struct MiTipo {
    valor: u32,
}

// Implementación de métodos de instancia para MiTipo
impl MiTipo {
    // Un método regular que toma una referencia inmutable a self
    fn metodo_regular(&self) {
        println!("Método regular: valor = {}", self.valor);
    }

    // Un método mutable que toma una referencia mutable a self
    fn metodo_mut(&mut self) {
        self.valor += 1;
        println!("Método mutable: nuevo valor = {}", self.valor);
    }
}

// Implementación de un método estático para MiTipo
impl MiTipo {
    // Un método estático que no toma una referencia a self
    fn metodo_estatico() {
        println!("Método estático llamado");
    }
}

// Implementación de un método asociado para MiTipo
impl MiTipo {
    // Un método asociado que toma un valor u32 y crea una nueva instancia de MiTipo
    fn nuevo(valor: u32) -> MiTipo {
        MiTipo { valor }
    }
}

// Implementación del trait MiTrait para MiTipo
impl MiTrait for MiTipo {
    // Implementación del método de trait
    fn metodo_de_trait(&self) -> u32 {
        self.valor * 2
    }
}

fn main() {
    // Creación de una instancia de MiTipo
    let mut mi_instancia = MiTipo { valor: 42 };

    // Llamada a métodos de instancia
    mi_instancia.metodo_regular();
    mi_instancia.metodo_mut();

    // Llamada a un método asociado
    let usando_asociado = MiTipo::nuevo(15);
    println!("Metodo asociado: {:?}",usando_asociado);

    // Llamada a un método estático
    MiTipo::metodo_estatico();

    // Llamada a un método de trait
    let resultado_trait = mi_instancia.metodo_de_trait();
    println!("Resultado del método de trait: {}", resultado_trait);
}

