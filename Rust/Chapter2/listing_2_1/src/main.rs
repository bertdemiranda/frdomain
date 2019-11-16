use bigdecimal::BigDecimal;
use std::str::FromStr;

type Amount   = BigDecimal;
type Duration = u32;

enum AccountType {
    SAVINGS,
    CHECKING,
}

struct Account {
    account_type: AccountType,
}

fn calculate_interest(account: Account, _period: Duration) -> Result<Amount, String> {
    match account.account_type {
        AccountType::SAVINGS => { 
            Ok(BigDecimal::from_str(&String::from("1.5")).unwrap()) 
        },
        _ => { Err(String::from("The account has to be a savings account.")) },
    }
}

fn main() {
    let dur = 1;

    let a   = Account{account_type: AccountType::SAVINGS};
    println!("Interest = {:?}", calculate_interest(a, dur));

    let b   = Account{account_type: AccountType::CHECKING};
    println!("Interest = {:?}", calculate_interest(b, dur));
}
