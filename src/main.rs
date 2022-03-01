mod my_games;
mod currency;

// use my_games::game;
// use currency::fiat_symbol;
// use currency::Fiat;
use currency::get_currency_list;


fn main() {
    // println!("{1} loves {0}, but {0} don't love {1}!", "Amanda", "Bob");
    // let tup: (i32, f64) = (300, 10.5);
    // let (x, y) = tup;
    // println!("inteiro {}, flutuante {}.", { x }, { y });
    // let result: i32 = sum(2, 5);
    // println!("Result of sum: {}", result);

    // println!("Dolar: {}\nReal: {}\nLibra: {}", fiat_symbol(Fiat::Dollar), fiat_symbol(Fiat::Real), fiat_symbol(Fiat::Pounds));

    // game::guess_game();

    for currency_symbol in get_currency_list() {
        println!("currency: {}", currency_symbol)
    }
}

// fn sum(num1: i32, num2: i32) -> i32 {
//     num1 + num2
// }
