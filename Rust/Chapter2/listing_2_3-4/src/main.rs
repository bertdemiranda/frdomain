#![allow(dead_code)]

use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

fn bigdec0() -> BigDecimal {
    BigDecimal::from_i32(0).unwrap()
}

type Amount   = BigDecimal;
type Duration = i32;
type AmountResult  = Result<Amount, String>;
type AmountResults = Vec<AmountResult>;

#[derive(Debug, Clone)]
struct Account {
    name:   String,
    number: String,
}

impl Account {
    fn new(name: &str, number: &str) -> Account {
        Account {
            name:   name.  to_string(),
            number: number.to_string()
        }
    }
}

#[derive(Debug, Clone)]
struct CheckingAccount {
    base_data: Account,
}

#[derive(Debug, Clone)]
struct InterestBearingAccount
{
    rate_of_interest: BigDecimal,
}

impl InterestBearingAccount {
    fn calc_interest(&self, period: Duration) -> Result<Amount, String> {
        let per: Option<BigDecimal> = FromPrimitive::from_i32(period);
        match per {
            Some(per) => Ok(per * &self.rate_of_interest),
            _         => Err(String::from("No valid period value"))
        }
    }
}

#[derive(Debug, Clone)]
struct SavingsAccount {
    base_data:     Account,
    interest_data: InterestBearingAccount,
}

impl SavingsAccount {
    fn new(name: &str, number: &str, rate_of_interest: BigDecimal) -> SavingsAccount {
        SavingsAccount {
            base_data:     Account::new(name, number), 
            interest_data: InterestBearingAccount {rate_of_interest},
        }
    }

    fn calc_interest(&self, _period: Duration) -> Result<Amount, String> {
        self.interest_data.calc_interest(_period)
    }
}

#[derive(Debug, Clone)]
struct MoneyMarketAccount {
    base_data:     Account,
    interest_data: InterestBearingAccount,
}

impl MoneyMarketAccount {
    fn new(name: &str, number: &str, rate_of_interest: BigDecimal) -> MoneyMarketAccount {
        MoneyMarketAccount {
            base_data:     Account::new(name, number), 
            interest_data: InterestBearingAccount {rate_of_interest},
        }
    }

    fn calc_interest(&self, _period: Duration) -> Result<Amount, String> {
        self.interest_data.calc_interest(_period)
    }
}

pub trait Interest {
    fn calculate_interest(&self, period: Duration) -> Result<Amount, String>;
}

impl Interest for SavingsAccount {
    fn calculate_interest(&self, period: Duration) -> Result<Amount, String> {
        self.calc_interest(period)
    }
}

impl Interest for MoneyMarketAccount {
    fn calculate_interest(&self, period: Duration) -> Result<Amount, String> {
        self.calc_interest(period)
    }
}

mod account_service {
    use crate::{Amount, Duration, Interest};
    pub fn calculate_interest<T: Interest>(account: &T, period: Duration) -> Result<Amount, String> {
        account.calculate_interest(period)
    }
}

fn main() {
    let dur = 10;

    let s = SavingsAccount::new("john", "acc2", bigdec("2.5"));
    println!("Interest = {:?}", s.calculate_interest(dur));
    println!("Interest = {:?}", account_service::calculate_interest(&s, dur));

    let m = MoneyMarketAccount::new("john", "acc3",bigdec("3.5"));
    println!("Interest = {:?}", m.calculate_interest(dur));
    println!("Interest = {:?}", account_service::calculate_interest(&m, dur));

    //--------------------------------------------------------------------------------------------------------------------

    let s1 = SavingsAccount::new("dg", "sb001", bigdec("0.5" ));
    let s2 = SavingsAccount::new("sr", "sb002", bigdec("0.75"));
    let s3 = SavingsAccount::new("ty", "sb003", bigdec("0.27"));
    
    let dur2 = 5;

    let r2: AmountResults = vec![s1.clone(), s2.clone(), s3.clone()]
                                .iter   ()
                                .map    (|acc| account_service::calculate_interest(acc, dur2))
                                .collect();
    println!("r2 = {:?}", r2);

    let r3 = vec![s1.clone(), s2.clone(), s3.clone()]
                                .iter   ()
                                .map    (|acc| account_service::calculate_interest(acc, dur2))
                                .fold   (bigdec0(), |a, e| if let Ok(amt) = e {amt + a} else {a});
    println!("r3 = {:?}", r3);


    let r4: AmountResults = vec![s1.clone(), s2.clone(), s3.clone()]
                                .iter   ()
                                .map    (|acc| account_service::calculate_interest(acc, dur2))
                                .filter (|amt| amt.is_ok())
                                .collect();
    println!("r4 = {:?}", r4);

    fn get_currency_balance(_a: &Account) -> AmountResult {
        Ok(bigdec("500"))
    }
    
    fn get_account_from(_no: &str) -> Result<Account, String> {
        Ok(Account::new("dg", "sb001"))
    }

    fn calculate_net_asset_value(_a: &Account, _balance: Amount) -> AmountResult {
        Ok(bigdec("1000"))
    }

    fn list_net_asset_value() -> Result<(Account, Amount), String> {
        let s = get_account_from("a1")?;
        let b = get_currency_balance(&s)?;
        let v = calculate_net_asset_value(&s, b)?;
        Ok((s,v))
    };
    println!("result = {:?}", list_net_asset_value());
}
