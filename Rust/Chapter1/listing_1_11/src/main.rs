#[allow(dead_code)]

extern crate chrono;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::prelude::*;

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
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Account {
    no: String,
    name: String,
    date_of_opening: Date<Local>,
    balance: Amount,
}

impl Account {
    fn new(no: &str, name: &str, date_of_opening: Date<Local>) -> Account {
        Account {no: String::from(no), name: String::from(name),
            date_of_opening,
            balance: Amount::new(0),
        }
    }
}

pub struct Customer {
    name: String,
}

impl Customer {
    fn new(name: &str) -> Customer {
        Customer {name: String::from(name)}
    }
}

// Verification of customers module
// --------------------------------
mod verifications {
    use crate::Customer;

    pub fn verify_record(_customer: Customer) -> Result<Customer, String> {
        //Ok(customer)
        Err(String::from("We don't know this person!!!"))
    }
}

// The AccountService functions
// ----------------------------
struct AccountService {}

trait AccountServiceFns {
    fn verify_customer(customer: Customer) -> Result<Customer, String>;
    fn open_checking_account(customer: Customer, effective_date: Date<Local>) -> Result<Account, String>;
}

use crate::verifications::verify_record;

impl AccountServiceFns for AccountService {

    fn verify_customer(customer: Customer) -> Result<Customer, String> {
        verify_record(customer)
    }

    fn open_checking_account(customer: Customer, effective_date: Date<Local>) -> Result<Account, String> {
        //..
        Ok(Account::new("acc1", &customer.name, effective_date))
    }
}

fn main() {
    let cust = Customer::new("customer1");
    match
    AccountService::verify_customer(cust)
            .and_then(|c| AccountService::open_checking_account(c, Local::now().date())) {
        Err(e) => println!("{}", e),
        Ok(_)  => (),
    }
}
