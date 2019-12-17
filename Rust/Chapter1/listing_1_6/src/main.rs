#![allow(dead_code)]

extern crate chrono;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::prelude::*;
use std::ops::{Add, Sub};

// The Account data
// ----------------
#[derive(Debug, PartialOrd, PartialEq)]
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

struct AccountService {}

trait AcctSrv {
    fn debit (a: Account, amount: Amount) -> Result<Account, String>;
    fn credit(a: Account, amount: Amount) -> Result<Account, String>;
}

impl AcctSrv for AccountService {
    fn debit(a: Account, amount: Amount) -> Result<Account, String> {
        if a.balance < amount {
            Err(String::from("Insufficient balance in account"))
        }
        else {
            Ok(Account{balance: a.balance - amount, ..a})
        }
    }

    fn credit(a: Account, amount: Amount) -> Result<Account, String> {
        Ok(Account{balance: a.balance + amount, ..a})
    }

}

fn main() {
    println!("Are all tests Ok?");
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn new_account_has_balance_0() {
        let a = new_account();
        assert!(a.balance == Amount::new(0));
    }

    #[test]
    fn credit_account() {
        let a   = new_account();
        let b   = AccountService::credit(a, Amount::new(1000)).unwrap();
        assert!(b.balance == Amount::new(1000));
    }

    #[test]
    fn debit_account_sufficient_balance() {
        let a   = new_account();
        let b   = AccountService::credit(a, Amount::new(1000)).unwrap();
        let c   = AccountService::debit (b, Amount::new( 500)).unwrap();
        assert_eq!(c.balance, Amount::new(500));
    }

    #[test]
    fn debit_account_insufficient_balance() {
        let a = new_account();
        let b = AccountService::credit(a, Amount::new(1000)).unwrap();
        assert_eq!(
            match AccountService::debit(b, Amount::new(1500)) {
                Err(errtext) => errtext,
                _ => String::from("An amount!")
            }, String::from("Insufficient balance in account")
        );
    }

    #[test]
    fn credit_debit_debit_account_sufficient_balance() {
        fn operations() -> Result<Amount, String> {
            let a = new_account();
            let b = AccountService::credit(a, Amount::new(1000))?;
            let c = AccountService::debit (b, Amount::new( 200))?;
            let d = AccountService::debit (c, Amount::new( 190))?;
            Ok(d.balance)
        }
        assert_eq!(operations(), Ok(Amount::new(610)));
    }

    #[test]
    fn credit_debit_debit_account_insufficient_balance() {
        fn operations() -> Result<Amount, String> {
            let a = new_account();
            let b = AccountService::credit(a, Amount::new(1000))?;
            let c = AccountService::debit (b, Amount::new(1500))?;
            let d = AccountService::debit (c, Amount::new( 200))?;
            Ok(d.balance)
        }
        assert_eq!(operations(), Err(String::from("Insufficient balance in account")));
    }

    fn new_account() -> Account {
        Account::new("a1", "John")
    }
}
