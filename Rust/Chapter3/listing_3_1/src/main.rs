#![allow(dead_code)]
extern crate chrono;

use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;

use chrono::prelude::*;

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

trait AccountService<Account, Amount, Balance> {
    fn open    (&self, no: &str, name: &str, 
                open_date: Option<Date<Local>>) ->                    Result<Account, &str>;
    fn close   (&self, account: Account, 
                close_date: Option<Date<Local>>) ->                   Result<Account, &str>;
    fn debit   (&self, account: Account, amount: &Amount) ->          Result<Account, &str>;
    fn credit  (&self, account: Account, amount: &Amount) ->          Result<Account, &str>;
    fn balance (&self, account: Account)                  ->          Result<Balance, &str>;
    fn transfer(&self, from: Account, to: Account, amount: Amount) -> Result<(Account, Account, Amount), &str> {
        let a = self.debit (from, &amount)?;
        let b = self.credit(to,   &amount)?;
        Ok((a, b, amount))
    } 
}

struct Account {
    balance: Balance,
}

type   Amount  = BigDecimal;
type   Balance = BigDecimal;

#[derive(Clone, Copy)]
struct AcctService{}

impl AccountService<Account, Amount, Balance> for AcctService {
    fn open    (&self, _no: &str, _name: &str, 
            _open_date: Option<Date<Local>>) ->                Result<Account, &str> {
        Ok(Account{balance: bigdec("0")})
    }
    fn close   (&self, account: Account, 
            _close_date: Option<Date<Local>>) ->               Result<Account, &str> {
        Ok(account)
    }
    fn debit   (&self, account: Account, amount: &Amount) ->  Result<Account, &str> {
        Ok(Account{balance: account.balance - amount, ..account})
    }
    fn credit  (&self, account: Account, amount: &Amount) ->  Result<Account, &str> {
        Ok(Account{balance: account.balance + amount, ..account})
    }
    fn balance (&self, account: Account)                  ->  Result<Balance, &str> {
        Ok(account.balance)
    }
}

fn main() {
    println!("Hello, AccountService!");
}

/// Do a credit and then a debit of the same amount.
/// Return the resulting account.
fn credit_debit(account_service: AcctService, account: Account, amount: f64) -> Result<Account, String> {
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
            let account_service = AcctService{};
            let a               = account_service.open("a0001", "me", Some(Local::now().date())).unwrap();
            let balance_before  = bigdec("1000000");

            if let Ok(b) = account_service.credit(a, &balance_before) {
                if let Ok(c) = credit_debit(account_service, b, amount) {
                    return account_service.balance(c) == Ok(balance_before);
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
            let account_service = AcctService{};
            let a               = account_service.open("a0001", "me", Some(Local::now().date())).unwrap();
            let balance_before  = bigdec("1000000");

            if let Ok(b) = account_service.credit(a, &balance_before) {
                if let Ok(c) = credit_debit(account_service, b, amount) {
                    prop_assert_eq!(account_service.balance(c), Ok(balance_before));
                    return Ok(());
                }
                prop_assert!(false);
            }
            prop_assert!(false);
        }
    }
}
