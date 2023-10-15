// inline mod
// mod garden {} 

// file
use crate::garden::vegetables::Asparagus;

pub mod garden;


// in the file mod.rs inside materials
mod materials;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
