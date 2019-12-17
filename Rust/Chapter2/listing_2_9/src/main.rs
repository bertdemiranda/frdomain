#![allow(dead_code)]
extern crate chrono;

use bigdecimal::{BigDecimal, FromPrimitive};
use std::ops::Add;
use chrono::prelude::*;

struct Account {
    no:   String,
    name: String,
    date_of_opening: Date<Local>,
    balance: Balance,
}

#[derive(Clone, Debug, PartialEq)]
struct Equity {
    isin: String,
    name: String,
    date_of_issue: Date<Local>,
    issue_currency: Currency,               // <<< Have to add this for get_market_value!
}

#[derive(Clone, Debug, PartialEq)]
struct FixedIncome {
    isin: String,
    name: String,
    date_of_issue: Date<Local>,
    issue_currency: Currency,
    nominal: BigDecimal,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Currency {
    USD,
    JPY,
}

#[derive(Clone, Debug, PartialEq)]
struct Amount {
    a: BigDecimal,
    c: Currency,
}

fn bigdec(i: i64) -> BigDecimal {
    BigDecimal::from_i64(i).unwrap()
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, that: Amount) -> Amount {
        assert!(&self.c == &that.c);
        Amount {
            a: &self.a + &that.a, 
            c: self.c
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Instrument {
    Ccy(Currency),
    Eqy(Equity),
    Fin(FixedIncome),
}

#[derive(Clone, Debug, PartialEq)]
struct Balance {
    amount: BigDecimal,
    ins:    Instrument,
    as_of:  Date<Local>
}

fn get_holding(account: Account) -> Amount {
    let get_market_value = |e: Equity, a: BigDecimal| {
        Amount{a: a, c: e.issue_currency}
    };

    let get_accrued_interest = |_i: String, c: Currency| {
        Amount{a: bigdec(300), c: c}
    };

    match account.balance {
        Balance{amount: a, ins: Instrument::Ccy(c), as_of: _} => Amount{a,c},
        Balance{amount: a, ins: Instrument::Eqy(e), as_of: _} => get_market_value(e,a),
        Balance{amount: a, ins: Instrument::Fin(f), as_of: _} => {
            Amount{a: f.nominal*a, c: f.issue_currency} + get_accrued_interest(f.isin, f.issue_currency)
        }
    }
}

fn main() {
    println!("Hello, holding!");
}
