extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Vou adivinha tuas parada ai, vacilao do caralho... da teu palpite ai, seu colhedor de canavial de rola");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("deu erro nessa merda aqui, mermao");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ce eh tao incompetente que nem sabe digitar numero, tenta dnv puta");
                continue;
            }
        };
        println!("Voce chutou: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Koe mermao sai da minha cabeca");
                break;
            }
            Ordering::Less => println!("Pequeno demais igual teu pau otario"),
            Ordering::Greater => println!("Grande demais igual a gorda da tua mae"),
        }
    }
}
