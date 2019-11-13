#[allow(dead_code)]
enum Account {}

#[allow(dead_code)]
enum Criteria<T> {
    Bar(T),
}

trait AccountRepository {
    fn query_account (account_no: String) ->          Option<Account>;
    fn query_accounts(criteria: Criteria<Account>) -> Vec<Account>;
    fn write         (accounts: Vec<Account>) ->      bool;
    fn delete        (account: Account) ->            bool;
}

fn main() {
    println!("Hello, Repository!");
}
