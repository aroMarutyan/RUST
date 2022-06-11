use hola::greet;
use rand::{Rng};


fn main() {
    greet();
    let x = rand::thread_rng().gen_range(0,100);
    println!("{}", x);
}
