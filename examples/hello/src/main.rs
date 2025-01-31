use hello::{english, russian, spanish};
use rand::Rng;

fn main() {
    english::greet();
    spanish::greet();
    russian::greet();

    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
}
