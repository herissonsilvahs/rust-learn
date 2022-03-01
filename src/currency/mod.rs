pub enum Fiat {
  Dollar,
  Real,
  Pounds,
}

pub fn fiat_symbol(fiat: Fiat) -> String {
  match fiat {
      Fiat::Dollar => String::from("$"),
      Fiat::Real => String::from("R$"),
      Fiat::Pounds => String::from("£")
  }
}

pub fn get_currency_list() -> Vec<String> {
  let mut currencies: Vec<String> = Vec::new();
  currencies.push(String::from("BRL"));
  currencies.push(String::from("USD"));
  currencies.push(String::from("GBP"));
  currencies
}
