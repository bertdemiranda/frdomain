#![allow(dead_code)]

use bigdecimal::BigDecimal;
use std::str::FromStr;

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

type Amount   = BigDecimal;
type Duration = u32;

struct Account {
    number: String,
    name:   String,
}

trait InterestBearingAccount {
    fn interest_rate(&self) -> Amount;
}

struct CheckingAccount {
    base_data: Account,
}

struct SavingsAccount {
    base_data:        Account,
    rate_of_interest: BigDecimal,
}

impl SavingsAccount {
    fn new(base_data: Account, rate_of_interest: BigDecimal) -> SavingsAccount {
        SavingsAccount {
            base_data, 
            rate_of_interest,
        }
    }
}

impl InterestBearingAccount for SavingsAccount {
    fn interest_rate(&self) -> Amount {
        self.rate_of_interest.clone()
    }
}

struct MoneyMarketAccount {
    base_data:        Account,
    rate_of_interest: BigDecimal,
}

impl MoneyMarketAccount {
    fn new(base_data: Account, rate_of_interest: BigDecimal) -> SavingsAccount {
        SavingsAccount {
            base_data, 
            rate_of_interest,
        }
    }
}

impl InterestBearingAccount for MoneyMarketAccount {
    fn interest_rate(&self) -> Amount {
        self.rate_of_interest.clone()
    }
}

fn calculate_interest(account: impl InterestBearingAccount, _period: Duration) -> Result<Amount, String> {
    Ok(account.interest_rate()) 
}

fn main() {
    let dur = 1;

    let s = SavingsAccount::new(
        Account {
            number: String::from("acc2"),
            name:   String::from("john"),
        },
        BigDecimal::from_str(&String::from("2.5")).unwrap()
    );
    println!("Interest Savings Account = {:?}", calculate_interest(s, dur));

    let m = MoneyMarketAccount::new(
        Account {
            number: String::from("acc3"),
            name:   String::from("john"),
        },
        BigDecimal::from_str(&String::from("3.5")).unwrap()
    );
    println!("Interest Moneymarket Account = {:?}", calculate_interest(m, dur));
}
