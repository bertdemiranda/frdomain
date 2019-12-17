#![allow(dead_code)]

extern crate chrono;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::prelude::*;
use std::ops::{Add, Sub};

// The Account data
// ----------------
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Amount {
    amount: BigDecimal,
}

impl Amount {
    pub fn new(amount: i64) -> Amount {
        Amount {
            amount: BigDecimal::from_i64(amount).unwrap()
        }
    }

    // pub fn get_amount(&self) -> BigDecimal {
    //     self.amount.clone()
    // }
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, amount: Amount) -> Amount {
        Amount {
            amount: &self.amount + amount.amount
        }
    }
}

impl Sub for Amount {
    type Output = Amount;
    fn sub(self, amount: Amount) -> Amount {
        Amount {
            amount: &self.amount - amount.amount
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Account {
    no: String,
    name: String,
    date_of_opening: Date<Local>,
    balance: Amount,
}

impl Account {
    fn new(no: &str, name: &str) -> Account {
        Account {no: String::from(no), name: String::from(name),
            date_of_opening: Local::now().date(),
            balance: Amount::new(0),
        }
    }
}

// The AccountService functions
// ----------------------------
mod account_service {
    use crate::Account;
    use crate::Amount;

    pub fn debit(a: Account, amount: Amount) -> Result<Account, String> {
        if a.balance < amount {
            Err(String::from("Insufficient balance in account"))
        }
        else {
            Ok(Account{balance: a.balance - amount, ..a})
        }
    }

    pub fn credit(a: Account, amount: Amount) -> Result<Account, String> {
        Ok(Account{balance: a.balance + amount, ..a})
    }

    fn generate_audit_log(a: Account, amount: Amount) -> Result<String, String> {
        Ok(format!("Debited {} from {}", amount.amount, a.name))
    }

    fn write(line: String) {
        println!("{}", line);
    }

    pub fn logged_debit(source: Account, amount: Amount) {
        // Longer:
        /* ------ 
        match 
            debit(source, amount.clone())
                .and_then(|b| generate_audit_log(b, amount.clone())) {
                    Ok(line) => write(line),
                    _        => (),
        };
        */
        // or shorter:
        // ----------
        if let Ok(line) = 
            debit(source, amount.clone())
                .and_then(|b| generate_audit_log(b, amount)) {
                    write(line);
        }
    }
}

fn main() {
    use account_service::{credit, logged_debit};

    let a = Account::new("a1", "Joe");
    let b = credit(a, Amount::new(1000)).unwrap();
    logged_debit(b, Amount::new(500));
}
