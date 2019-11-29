#![allow(dead_code)]

use bigdecimal::BigDecimal;
use std::str::FromStr;
use futures::executor::block_on;
use futures::try_join;

fn bigdec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

struct SavingsAccount {
    rate: BigDecimal,
}

fn calculate_interest(account: &SavingsAccount, balance: &BigDecimal) -> Result<BigDecimal, String> {
    if account.rate == bigdec("0") {
        Err(String::from("Interest Rate nog found"))
    } else {
        Ok(bigdec("1000"))
    }
}

fn get_currency_balance(account: &SavingsAccount) -> Result<BigDecimal, String> {
    Ok(bigdec("1000"))
} 

fn calculate_net_asset_value(account: &SavingsAccount, ccy_balance: &BigDecimal, interest: BigDecimal) -> Result<BigDecimal, String> {
    Ok(ccy_balance + interest + bigdec("200"))
}

async fn do_calculate(account: &SavingsAccount) -> Result<(&SavingsAccount, BigDecimal), String> {
    let b = get_currency_balance(account)?;
    let i = calculate_interest(account, &b)?;
    let v = calculate_net_asset_value(account, &b, i)?;
    Ok((account, v))
}

fn main() {
    let s1 = SavingsAccount {
        rate: bigdec("0.2"),
    };

    match block_on(do_calculate(&s1)) {
        Ok((acc, nv)) => println!("Hello, net assets of {}!", nv),
        Err(e)        => println!("Error {}!", e)
    }
}
