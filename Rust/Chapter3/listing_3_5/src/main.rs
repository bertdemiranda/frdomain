#![allow(dead_code)]

mod account {
    extern crate chrono;

    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    
    use chrono::prelude::*;
    
    pub fn bigdec(s: &str) -> BigDecimal {
        BigDecimal::from_str(s).unwrap()
    }
    
    type   Amount  = BigDecimal;
    
    #[derive(Clone, Debug, Default)]
    pub struct Balance {
        amount: Amount
    }

    impl Balance {
        pub fn amount(&self) -> Amount {
            self.amount.clone()
        }
    }
    
    type LocalDate = Date<Local>;
    pub fn today() -> LocalDate {
        Local::now().date()
    }
    
    #[derive(Clone, Debug, Default)]
    pub struct Account {
        no:            String,
        name:          String,
        date_of_open:  Option<LocalDate>,
        date_of_close: Option<LocalDate>,
        balance:       Balance,
    }

    #[derive(Clone, Debug, Default)]
    pub struct CheckingAccount {
        account: Account,
    }

    impl CheckingAccount {
        pub fn new(no: &str, name: &str,
                   open_date:  Option<LocalDate>, 
                   close_date: Option<LocalDate>, 
                   balance:    Balance) -> Result<CheckingAccount, String> {
            if let Ok((od, cd)) = close_date_check(open_date, close_date) {
                Ok(CheckingAccount {
                    account: Account {
                        no:   String::from(no), 
                        name: String::from(name), 
                        date_of_open: Some(od), 
                        date_of_close: cd,
                        balance: balance
                    }
                })
            }
            else {
                Err(String::from("Cannot create a checking account"))
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct SavingsAccount {
        account: Account,
        rate_of_interest: Amount,
    }

    impl SavingsAccount {
        pub fn new(no: &str, name: &str, rate: BigDecimal, 
                   open_date:  Option<LocalDate>, 
                   close_date: Option<LocalDate>, 
                   balance:    Balance) -> Result<SavingsAccount, String> {
            if let Ok((od, cd)) = close_date_check(open_date, close_date) {
                if rate <= bigdec("0") {
                    Err(format!("Interest rate {} must be > 0", rate))
                }
                else {
                    Ok(SavingsAccount {
                        account: Account {
                            no:   String::from(no), 
                            name: String::from(name), 
                            date_of_open: Some(od), 
                            date_of_close: cd,
                            balance: balance
                        },
                        rate_of_interest: rate
                    })
                }
            }
            else {
                Err(String::from("Cannot create a checking account"))
            }
        }
    }

    fn close_date_check(open_date: Option<LocalDate>, 
                        close_date: Option<LocalDate>) -> 
                            Result<(LocalDate, Option<LocalDate>), String> {
        let od = match open_date {
            Some(d) =>  d,
            _       =>  today()
        };

        let cd = match close_date {
            Some(d) =>  {
                if d < od { return Err(format!("Close date {} cannot be earlier than open date {}", d, od)); }
                else      { Some(d) }
            }
            _       => None
        };
        Ok((od, cd))
    }
}

use account::*;

fn main() {
    let ca = CheckingAccount::new("ca001", "me", Some(today()), None, Balance::default());
    println!("Hello, checking account {:?}!", ca);

    let sa = SavingsAccount::new("ca001", "me", bigdec("0.1"), Some(today()), None, Balance::default());
    println!("Hello, savings account {:?}!", sa);
}
