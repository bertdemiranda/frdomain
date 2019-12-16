#![allow(dead_code)]

use bigdecimal::BigDecimal;
use std::str::FromStr;

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

type Amount   = BigDecimal;
type Duration = i32;
type AmountResult  = Result<Amount, String>;
type AmountResults = Vec<AmountResult>;

#[derive(Debug, Clone)]
struct Balance {
    amount: BigDecimal,
}

impl Balance {
    fn new(b: &str) -> Balance {
        Balance {amount: bigdec(b)}
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
        if i < &bigdec("1000") {
            i.clone()
        } else {
            i - bigdec("0.1") * i
        }};
        
    let a1 = SavingsAccount::new("a-0001", "ibm",    Balance::new( "100000"), bigdec("0.12"));
    let a2 = SavingsAccount::new("a-0002", "google", Balance::new("2000000"), bigdec("0.13"));
    let a3 = SavingsAccount::new("a-0003", "chase",  Balance::new( "125000"), bigdec("0.15"));

    let accounts = vec![a1, a2, a3];

    let r1: Vec<BigDecimal> = accounts.iter().map(|a| calculate_interest(a)).map(|i| deduct_tax(i)).collect();
    println!("r1 = {:?}", r1);

    let r2: Vec<BigDecimal> = accounts.iter().map(|a| deduct_tax(calculate_interest(a))).collect(); // how to use and_then?
    println!("r2 = {:?}", r2);
}
