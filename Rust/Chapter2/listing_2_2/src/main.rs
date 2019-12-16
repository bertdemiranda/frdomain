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

struct CheckingAccount {
    base_data: Account,
}

struct InterestBearingAccount
{
    rate_of_interest: BigDecimal,
}

impl InterestBearingAccount {
    fn calc_interest(&self, _period: Duration) -> Result<Amount, String> {
        Ok(bigdec("1.5")) 
    }
}

struct SavingsAccount {
    base_data:     Account,
    interest_data: InterestBearingAccount,
}

impl SavingsAccount {
    fn new(base_data: Account, rate_of_interest: BigDecimal) -> SavingsAccount {
        SavingsAccount {
            base_data, 
            interest_data: InterestBearingAccount { rate_of_interest },
        }
    }

    fn calculate_interest(&self, _period: Duration) -> Result<Amount, String> {
        self.interest_data.calc_interest(_period)
    }
}

struct MoneyMarketAccount {
    base_data:     Account,
    interest_data: InterestBearingAccount,
}

impl MoneyMarketAccount {
    // fn new(base_data: Account, rate_of_interest: BigDecimal) -> MoneyMarketAccount {
    //     // ...
    // }

    fn calculate_interest(&self, _period: Duration) -> Result<Amount, String> {
        self.interest_data.calc_interest(_period)
    }
}

fn main() {
    let dur = 1;

    let b = SavingsAccount::new(
        Account {
            number: String::from("acc2"),
            name:   String::from("john"),
        },
        BigDecimal::from_str(&String::from("2.5")).unwrap()
    );
    println!("Interest = {:?}", b.calculate_interest(dur));
}
