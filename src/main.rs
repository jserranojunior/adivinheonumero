
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Jogo: Advinhe o número");
    println!("Informe um número");

let secret_number:u32 = rand::thread_rng()
        .gen_range(1..=100);

    let mut guesses: u32 = 0;

    loop{
        
        let mut guess:String = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Falha ao executar o número");
    
        println!("Você chutou o número: {guess}");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_) => continue,
        };
        guesses += 1;
        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("Você ganhou");
                break
            },
            Ordering::Greater => println!("O núemro é menor"),
            Ordering::Less => println!("O núemro é maior"),

        }
    }
    print!("Voce fez {guesses} tentativas!")
  /*   else if secret_number > guess{
        println!("Você chutou alto");

    }else{
        println!("Você acertou");

    } */

}
