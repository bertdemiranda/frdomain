extern crate chrono;

use bigdecimal::BigDecimal;
use chrono::prelude::*;

type Amount  = BigDecimal;
type Bank    = String;
type Address = String;
type Date    = DateTime<Local>;

#[allow(dead_code)]
struct AccountData {
    no: String,
    name: String,
    bank: Bank,
    address: Address,
    date_of_opening: Date,
    date_of_close: Option<Date>,
}

#[allow(dead_code)]
enum Account {
    CheckingAccount   {
        basedata: AccountData,
    },
    SavingsAccount    {
        basedata: AccountData,
        rate_of_interest: BigDecimal,
    },
    MoneyMarketAccount{
        basedata: AccountData,
        /* data */
    },
}

trait AccountService {
    fn transfer(from: Account, to: Account, amount: Amount) -> Option<Amount>;
}

impl AccountService for Account {
    fn transfer(_from: Account, _to: Account, _amount: Amount) -> Option<Amount> {
        None
    }
}

fn main() {
    println!("Hello, AccountService!");
}
