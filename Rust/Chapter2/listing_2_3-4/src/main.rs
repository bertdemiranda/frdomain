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

trait InterestBearingAccount {
    fn interest_rate(&self) -> Amount;
}

#[derive(Debug, Clone)]
struct CheckingAccount {
    base_data: Account,
}

#[derive(Debug, Clone)]
struct SavingsAccount {
    base_data:        Account,
    rate_of_interest: BigDecimal,
}

impl SavingsAccount {
    fn new(name: &str, number: &str, rate_of_interest: BigDecimal) -> SavingsAccount {
        SavingsAccount {
            base_data:     Account::new(name, number), 
            rate_of_interest,
        }
    }

    // fn calc_interest(&self, _period: Duration) -> Result<Amount, String> {
    //     self.interest_data.calc_interest(_period)
    // }
}

impl InterestBearingAccount for SavingsAccount {
    fn interest_rate(&self) -> Amount {
        self.rate_of_interest.clone()
    }
}

#[derive(Debug, Clone)]
struct MoneyMarketAccount {
    base_data:        Account,
    rate_of_interest: BigDecimal,
}

impl MoneyMarketAccount {
    fn new(name: &str, number: &str, rate_of_interest: BigDecimal) -> MoneyMarketAccount {
        MoneyMarketAccount {
            base_data:     Account::new(name, number), 
            rate_of_interest,
        }
    }

    // fn calc_interest(&self, _period: Duration) -> Result<Amount, String> {
    //     self.interest_data.calc_interest(_period)
    // }
}

impl InterestBearingAccount for MoneyMarketAccount {
    fn interest_rate(&self) -> Amount {
        self.rate_of_interest.clone()
    }
}

struct AccountService {}

trait AccountServiceFns {
    fn calculate_interest(account: &impl InterestBearingAccount, period: Duration) -> Result<Amount, String>;
}

impl  AccountServiceFns for AccountService {
    fn calculate_interest(account: &impl InterestBearingAccount, _period: Duration) -> Result<Amount, String> {
        Ok(account.interest_rate())
    }
}

fn main() {
    let dur = 10;

    let s = SavingsAccount::new("john", "acc2", bigdec("2.5"));
    println!("Interest = {:?}", AccountService::calculate_interest(&s, dur));

    let m = MoneyMarketAccount::new("john", "acc3",bigdec("3.5"));
    println!("Interest = {:?}", AccountService::calculate_interest(&m, dur));

    //--------------------------------------------------------------------------------------------------------------------

    let s1 = SavingsAccount::new("dg", "sb001", bigdec("0.5" ));
    let s2 = SavingsAccount::new("sr", "sb002", bigdec("0.75"));
    let s3 = SavingsAccount::new("ty", "sb003", bigdec("0.27"));
    
    let dur2 = 5;

    let r2: AmountResults = vec![s1.clone(), s2.clone(), s3.clone()]
                                .iter   ()
                                .map    (|acc| AccountService::calculate_interest(acc, dur2))
                                .collect();
    println!("r2 = {:?}", r2);

    let r3 = vec![s1.clone(), s2.clone(), s3.clone()]
                                .iter   ()
                                .map    (|acc| AccountService::calculate_interest(acc, dur2))
                                .fold   (bigdec0(), |a, e| if let Ok(amt) = e {amt + a} else {a});
    println!("r3 = {:?}", r3);


    let r4: AmountResults = vec![s1.clone(), s2.clone(), s3.clone()]
                                .iter   ()
                                .map    (|acc| AccountService::calculate_interest(acc, dur2))
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
