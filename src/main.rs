use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Adivinhe o número!");
    let mut guesses: u32 = 0;
    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);
    // println!("O número secreto é: {secret_number}");

   loop {
        println!("Por favor, insira seu palpite.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");


        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        guesses += 1;
        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            },
            Ordering::Greater => println!("Seu palpite foi maior que o número secreto!"),
            Ordering::Less => println!("Seu palpite foi menor que o número secreto!"),
        };

        println!("Número de tentativas: {}", guesses);
   }
}
