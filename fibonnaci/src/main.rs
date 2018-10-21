use std::io;

fn main() {
    let mut anterior1 = 1;
    let mut anterior2 = 1;
    let mut somatorio = anterior1 + anterior2;

    println!("os numeros 1, 2 e 3 ja foram gerados por padrao. entre com n maior q 3");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Nao deu certo ler");
    let entrada: u32 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => 3,
    };
    for number in 1..entrada{
        anterior1 = anterior2;
        anterior2 = somatorio;
        somatorio = anterior1 + anterior2;
    }
    println!("n = {} ->: {}",entrada, somatorio);

}
