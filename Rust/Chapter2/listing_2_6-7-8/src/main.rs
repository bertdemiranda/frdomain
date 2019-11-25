#![allow(dead_code)]
extern crate chrono;

use bigdecimal::BigDecimal;
use chrono::prelude::*;

//-------------------

enum Currency {
    USD,
    AUD,
    EUR,
    INR,
}

//-------------------

type A = i32;
type B = i64;

enum Either {
    Left (A),
    Right(B),
}

//-------------------

struct Account {
    number: String,
    name:   String,
}

struct CheckingAccount {
    account: Account,
    date_of_opening: Date<Local>,
}

struct SavingsAccount {
    account: Account,
    date_of_opening: Date<Local>,
    rate_of_interest: BigDecimal,
}

//-------------------

fn main() {
    println!("Hello, accounts!");
}
