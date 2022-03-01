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
