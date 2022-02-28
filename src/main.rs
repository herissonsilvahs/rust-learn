use rand::Rng;
use std::cmp::Ordering;
use std::io;

enum Fiat {
    Dollar,
    Real,
    Pounds,
}

fn fiat_symbol(fiat: Fiat) -> String {
    match fiat {
        Fiat::Dollar => String::from("$"),
        Fiat::Real => String::from("R$"),
        Fiat::Pounds => String::from("£")
    }
}

fn guess_game() {
    let secret_number = rand::thread_rng().gen_range(1..51);
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            println!("Bye!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn main() {
    // println!("{1} loves {0}, but {0} don't love {1}!", "Amanda", "Bob");
    // let tup: (i32, f64) = (300, 10.5);
    // let (x, y) = tup;
    // println!("inteiro {}, flutuante {}.", { x }, { y });
    // let result: i32 = sum(2, 5);
    // println!("Result of sum: {}", result);

    println!("Dolar: {}\nReal: {}\nLibra: {}", fiat_symbol(Fiat::Dollar), fiat_symbol(Fiat::Real), fiat_symbol(Fiat::Pounds))
}

// fn sum(num1: i32, num2: i32) -> i32 {
//     num1 + num2
// }
