#![allow(dead_code)]

use bigdecimal::BigDecimal;
use std::str::FromStr;

type Amount   = BigDecimal;
type Duration = u32;

struct Account {
    number: String,
    name:   String,
}

trait Interest {
    fn calculate_interest(&self, _period: Duration) -> Result<Amount, String> {
        Err(String::from("The account has to be a savings account."))
    }
}

struct CheckingAccount {
    base_data: Account,
}

impl Interest for CheckingAccount {}

struct SavingsAccount {
    base_data: Account,
    rate_of_interest: BigDecimal,
}

impl Interest for SavingsAccount {
    fn calculate_interest(&self, _period: Duration) -> Result<Amount, String> {
        let _rate = &self.rate_of_interest;
        Ok(BigDecimal::from_str(&String::from("1.5")).unwrap()) 
    }
}

fn main() {
    let dur = 1;

    let a = CheckingAccount{
        base_data: Account {
            number: String::from("acc1"),
            name:   String::from("john"),
        },
    };
    println!("Interest = {:?}", a.calculate_interest(dur));

    let b = SavingsAccount{
        base_data: Account {
            number: String::from("acc2"),
            name:   String::from("john"),
        },
        rate_of_interest: BigDecimal::from_str(&String::from("2.5")).unwrap(),
    };
    println!("Interest = {:?}", b.calculate_interest(dur));
}
