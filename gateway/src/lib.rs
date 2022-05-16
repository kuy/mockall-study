use domain::Coffee;
use usecase::port::coffee_port::CoffeePort;

#[mockall_double::double]
use driver::awesome_coffee_machine::AwesomeCoffeeMachine;

pub struct CoffeeGateway {
    driver: AwesomeCoffeeMachine
}

#[async_trait::async_trait]
impl CoffeePort for CoffeeGateway {
    async fn get_coffee(&self, name: String) -> Coffee {
        let liquid = self.driver.brew_coffee(name.clone()).await;
        Coffee { name, amount: liquid.amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use driver::awesome_coffee_machine::Liquid;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_find_by_hoge() {
        let mut driver = AwesomeCoffeeMachine::new();
        driver
            .expect_brew_coffee()
            .with(eq("Ethiopia".to_string()))
            .times(1)
            .return_once(|_| Liquid { amount: 200 });

        let target = CoffeeGateway { driver };

        let actual = target.get_coffee("Ethiopia".into()).await;
        let expected = Coffee { name: "Ethiopia".into(), amount: 200 };

        assert_eq!(actual, expected);
    }
}
