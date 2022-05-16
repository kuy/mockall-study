use domain::Coffee;

#[async_trait::async_trait]
#[mockall::automock]
pub trait CoffeePort {
  async fn get_coffee(&self, name: String) -> Coffee;
}
