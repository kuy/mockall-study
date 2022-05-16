use domain::Coffee;

#[mockall::automock]
pub trait CoffeePort {
  fn get_coffee(&self, name: String) -> Coffee;
}
