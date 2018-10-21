use std::io;

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("no index {} o valor eh {}", index, a[index]);
        index = index + 1;
    }

    let mut entrada = String::new();
    println!("digite a temperatura");
    io::stdin().read_line(&mut entrada)
        .expect("input com erro");
    let entrada: f64 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0
    };
    let temperatura = entrada;
    println!("digite 0 para converter de  F pra C, ou outro valor \
        p converter C pra F");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada)
        .expect("problema na leitura");
    let entrada: u8 = match entrada.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    } as u8;

    let temperatura_formatada = convertTemperature(temperatura, entrada);
    println!("valor convertido {}", temperatura_formatada )
}

fn convertTemperature(entrada: f64, tipo: u8) -> f64 {
    // de F pra C
    if tipo == 0 {
        (entrada - 32.0)/ 1.8
    } else {
        entrada * 1.8 + 32.0
    }
}
