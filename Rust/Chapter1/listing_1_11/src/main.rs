extern crate chrono;

use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::prelude::*;

// The Account data
// ----------------
#[allow(dead_code)]
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

    pub fn add(&self, amount: Amount) -> Amount {
        Amount {
            amount: &self.amount + amount.amount
        }
    }

    pub fn sub(&self, amount: Amount) -> Amount {
        Amount {
            amount: &self.amount - amount.amount
        }
    }

    // pub fn get_amount(&self) -> BigDecimal {
    //     self.amount.clone()
    // }
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Account {
    no: String,
    name: String,
    date_of_opening: Date<Local>,
    //balance: Amount,
}

// impl Account {
//     fn new(no: String, name: String) -> Account {
//         Account {no, name,
//             date_of_opening: Local::now().date(),
//             balance: Amount::new(0),
//         }
//     }
// }

pub struct Customer {
    name: String,
}

// Verification of customers module
// --------------------------------
mod verifications {
    use crate::Customer;

    pub fn verify_record(customer: Customer) -> Result<Customer, String> {
        //Ok(customer)
        Err(String::from("Don't know this person!"))
    }
}

// The AccountService functions
// ----------------------------
mod account_service {
    use chrono::prelude::*;
    use crate::Account;
    use crate::Amount;
    use crate::Customer;
    use crate::verifications::verify_record;

    pub fn verify_customer(customer: Customer) -> Result<Customer, String> {
        verify_record(customer)
    }

    pub fn open_checking_account(customer: Customer, effective_date: Date<Local>) -> Result<Account, String> {
        //..
        let no = String::from("acc1");
        Ok(Account {no, name: customer.name, date_of_opening: effective_date})
    }
}

fn main() {
    use account_service::{verify_customer, open_checking_account};

    let cust = Customer { name: String::from("customer1") };
    match
        verify_customer(cust)
            .and_then(|c| open_checking_account(c, Local::now().date())) {
        Err(e) => println!("{}", e),
        Ok(_)  => (),
    }
}