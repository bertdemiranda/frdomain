#![allow(dead_code)]
extern crate chrono;

use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;

use chrono::prelude::*;

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

trait AccountServiceFns<Account, Amount, Balance> {
    fn open    (&self, no: &str, name: &str, 
                opening_date: Option<Date<Local>>) ->                 Result<Account, String>;
    fn close   (&self, account: Account, 
                close_date: Option<Date<Local>>) ->                   Result<Account, String>;
    fn debit   (&self, account: Account, amount: &Amount) ->          Result<Account, String>;
    fn credit  (&self, account: Account, amount: &Amount) ->          Result<Account, String>;
    fn balance (&self, account: Account)                  ->          Result<Balance, String>;
    fn transfer(&self, from: Account, to: Account, amount: Amount) -> Result<(Account, Account, Amount), String> {
        let a = self.debit (from, &amount)?;
        let b = self.credit(to,   &amount)?;
        Ok((a, b, amount))
    } 
}

struct Account {
    no:            String,
    name:          String,
    date_of_open:  Date<Local>,
    date_of_close: Option<Date<Local>>,
    balance:       Balance,
}

type   Amount  = BigDecimal;

fn today() -> Date<Local> {
    Local::now().date()
}

#[derive(Default)]
struct Balance {
    amount: Amount
}

use std::ops::{Add, Sub};

impl Add<Amount> for Balance {
    type Output = Balance;
    fn add(self, amount: Amount) -> Balance {
        Balance{amount: self.amount + amount}
    }
}

impl Sub<Amount> for Balance {
    type Output = Balance;
    fn sub(self, amount: Amount) -> Balance {
        Balance{amount: self.amount - amount}
    }
}


#[derive(Clone, Copy)]
struct AccountService{}

impl AccountServiceFns<Account, Amount, Balance> for AccountService {
    fn open    (&self, no: &str, name: &str, 
            opening_date: Option<Date<Local>>) ->             Result<Account, String> {
        if no.is_empty() || name.is_empty() {
            return Err(String::from("Account no or name cannot be blank"));
        }
        else if opening_date.is_none() || (opening_date < Some(today())) {
            return Err(String::from("Cannot open account in the past"));
        }
        else {
            let acct = Account{no: String::from(no), name: String::from(name), 
                                date_of_open: opening_date.unwrap(),
                                date_of_close: None,
                                balance: Balance::default()}; 
            return Ok(acct);
        }
    }
    fn close   (&self, account: Account, 
            close_date: Option<Date<Local>>) ->               Result<Account, String> {
        if close_date.is_none() || close_date < Some(account.date_of_open) {
            return Err(format!("Close date {:?} cannot be before opening date {:?}", close_date, account.date_of_open));
        }
        else {
            let acct = Account{date_of_close: close_date, ..account};
            return Ok(acct);
        }
    }
    fn debit   (&self, a: Account, amount: &Amount) ->  Result<Account, String> {
        if a.balance.amount < *amount {
            return Err(String::from("Insufficient balance"));
        }
        Ok(Account{balance: a.balance - amount.clone(), ..a})
    }
    fn credit  (&self, a: Account, amount: &Amount) ->  Result<Account, String> {
        Ok(Account{balance: a.balance + amount.clone(), ..a})
    }
    fn balance (&self, a: Account)                  ->  Result<Balance, String> {
        Ok(a.balance)
    }
}

fn main() {
    println!("Hello, AccountServiceFns!");
}

/// Do a credit and then a debit of the same amount.
/// Return the resulting account.
fn credit_debit(account_service: AccountService, account: Account, amount: f64) -> Result<Account, String> {
    let bigdec_amount = BigDecimal::from_f64(amount).unwrap();
    let a = account_service.credit(account, &bigdec_amount)?;
    let b = account_service.debit (a,       &bigdec_amount)?;
    Ok(b)
}

#[macro_use]
extern crate quickcheck;
#[cfg(test)]
mod quickcheck_tests {
    use crate::*;

    quickcheck! {
        fn credit_and_debit_with_same_amount_in_sequence_retain_the_same_balance(amount: f64) -> bool {
            // Use an f64 amount here, because trait Arbitrary is not implemented for type BigDecimal in quickcheck.
            let account_service = AccountService{};
            let a               = account_service.open("a0001", "me", Some(today())).unwrap();
            let amount_before   = bigdec("1000000");

            if let Ok(b) = account_service.credit(a, &amount_before) {
                if let Ok(c) = credit_debit(account_service, b, amount) {
                    if let Ok(balance_new) = account_service.balance(c) {
                        return balance_new.amount == amount_before;
                    }
                }
            }
            false
        }
    }
}

#[cfg(test)]
mod proptest_tests {
    use crate::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn credit_and_debit_with_same_amount_in_sequence_retain_the_same_balance(amount: f64) {
            // Use an f64 amount here, because trait Arbitrary is not implemented for type BigDecimal in proptest.
            let account_service = AccountService{};
            let a               = account_service.open("a0001", "me", Some(today())).unwrap();
            let amount_before   = bigdec("1000000");

            if let Ok(b) = account_service.credit(a, &amount_before) {
                if let Ok(c) = credit_debit(account_service, b, amount) {
                    if let Ok(balance_new) = account_service.balance(c) {
                        prop_assert_eq!(balance_new.amount, amount_before);
                        return Ok(());
                    }
                    prop_assert!(false);
                }
                prop_assert!(false);
            }
            prop_assert!(false);
        }
    }
}
