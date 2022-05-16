pub struct AwesomeCoffeeMachine;

#[mockall::automock]
impl AwesomeCoffeeMachine {
    pub async fn brew_coffee(&self, _name: String) -> Liquid {
        Liquid { amount: 120 }
    }
}

#[derive(Debug)]
pub struct Liquid {
    pub amount: i32
}
