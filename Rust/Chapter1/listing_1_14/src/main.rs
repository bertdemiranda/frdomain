use futures::executor::block_on;
use futures::try_join;

type Amount = i32;
type AmountResult = Result<Amount, String>;

async fn get_currency_balance() -> AmountResult {
    //Ok(100)
    Err(String::from("Error calculating the currency balance"))
}

async fn get_equity_balance() -> AmountResult {
    Ok(200)
}

async fn get_debt_balance() -> AmountResult {
    Ok(300)
}

async fn get_loan_information() -> AmountResult {
    Ok(400)
}

async fn get_retirementfund_balance() -> AmountResult {
    Ok(500)
}

async fn get_portfolio() -> Result<(Amount, Amount, Amount, Amount, Amount), String> {
    try_join!(
        get_currency_balance(), 
        get_equity_balance(),
        get_debt_balance(),
        get_loan_information(),
        get_retirementfund_balance()
    )
}

fn main() {
    let pfresult = block_on(get_portfolio());
    match pfresult {
        Ok(pf) => {
            let pf_total = pf.0 + pf.1 + pf.2 + pf.3 + pf.4;
            println!("Hello, balance: {}!", pf_total);
        }
        Err(e) => {
            println!("Hello, balance error: {}!", e);
        }
    }
}
