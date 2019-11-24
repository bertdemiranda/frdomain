#![allow(dead_code)]

use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;

type Amount   = BigDecimal;
type Duration = i32;
type AmountResult  = Result<Amount, String>;
type AmountResults = Vec<AmountResult>;

#[derive(Debug, Clone)]
struct Balance {
    amount: BigDecimal,
}

impl Balance {
    fn new_i32(b: i32) -> Balance {
        Balance {amount: BigDecimal::from_i32(b).unwrap()}
    }
}

#[derive(Debug, Clone)]
struct Account {
    no:      String,
    name:    String,
    balance: Balance,
}

impl Account {
    fn new(no: &str, name: &str, balance: Balance) -> Account {
        Account {
            no:      no.  to_string(),
            name:    name.to_string(),
            balance,
        }
    }
}

#[derive(Debug, Clone)]
struct SavingsAccount {
    base_data: Account,
    rate:      BigDecimal,
}

impl SavingsAccount {
    fn new(no: &str, name: &str, balance: Balance, rate: BigDecimal) -> SavingsAccount {
        SavingsAccount {
            base_data:     Account::new(no, name, balance), 
            rate,
        }
    }
    fn balance(&self) -> Balance {
        self.base_data.balance.clone()
    }
}

fn main() {

    let calculate_interest = 
        |a: &SavingsAccount|
        {&a.balance().amount * &a.rate};


    let deduct_tax =
        |interest: BigDecimal| {
        let i = &interest;
        if i < &BigDecimal::from_i32(1000).unwrap() {
            i.clone()
        } else {
            i - BigDecimal::from_str("0.1").unwrap() * i
        }};
        
    let a1 = SavingsAccount::new("a-0001", "ibm",    Balance::new_i32( 100000), BigDecimal::from_str("0.12").unwrap());
    let a2 = SavingsAccount::new("a-0002", "google", Balance::new_i32(2000000), BigDecimal::from_str("0.13").unwrap());
    let a3 = SavingsAccount::new("a-0003", "chase",  Balance::new_i32( 125000), BigDecimal::from_str("0.15").unwrap());

    let accounts = vec![a1, a2, a3];

    let r1: Vec<BigDecimal> = accounts.iter().map(|a| calculate_interest(a)).map(|i| deduct_tax(i)).collect();
    println!("r1 = {:?}", r1);

    let r2: Vec<BigDecimal> = accounts.iter().map(|a| deduct_tax(calculate_interest(a))).collect(); // how to use and_then?
    println!("r2 = {:?}", r2);
}
