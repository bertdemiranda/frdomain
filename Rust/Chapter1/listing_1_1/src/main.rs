#[allow(dead_code)]
enum Account {
    CheckingAccount   (/* data */),
    SavingsAccount    (/* data */),
    MoneyMarketAccount(/* data */),
}

#[allow(dead_code)]
impl Account {
    fn apply(/* parameters */) -> Account {
        // instantiate Checking, Savings or MoneyMarket account
        // depending on parameters
        Account::CheckingAccount()
    }
}

fn main() {
    println!("Hello, Account!");
}
