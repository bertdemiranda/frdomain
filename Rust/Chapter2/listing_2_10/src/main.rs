#![allow(dead_code)]
extern crate chrono;

use bigdecimal::BigDecimal;
use std::str::FromStr;
use chrono::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct SavingsAccount {
    number: String,
    name: String,
    date_of_opening: Date<Local>,
    rate_of_interest: BigDecimal,
}

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

fn main() {
    let today = Local::now().date();

    let a1 = SavingsAccount {
        number: String::from("a-123"),
        name:   String::from("google"),
        date_of_opening: today,
        rate_of_interest: bigdec("0.2"),
    };
    println!("a1 = {:?}", a1);

    let a2 = SavingsAccount {
        rate_of_interest: bigdec("0.15"),
        ..a1.clone()
    };
    println!("a2 = {:?}", a2);
}
