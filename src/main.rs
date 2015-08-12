#[macro_use(accepts, to_sql_checked)]
extern crate postgres;
extern crate chrono;
extern crate byteorder;
extern crate num;

mod currency;

use postgres::{Connection, SslMode};
use chrono::NaiveDateTime;
use currency::Currency;


// Define the data in the database.
struct Account {
    id: i32,
    name: String,
    kind: String,
}

struct Balance {
    id: i32,
    account: i32,
    as_of: chrono::NaiveDateTime,
    balance: Currency
}

struct Deposit {
    id: i32,
    account: i32,
    at: chrono::NaiveDateTime,
    amount: Currency
}

fn main() {
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let stmt = conn.prepare("
        SELECT DISTINCT ON (balance.account) accounts.name, balance.balance
        FROM balance
        INNER JOIN accounts ON balance.account=accounts.id
        ORDER BY balance.account, balance.as_of DESC"
    ).unwrap();

    println!("Balances:");

    for row in stmt.query(&[]).unwrap() {
        let name: String = row.get(0);
        let balance: Currency = row.get(1);
        println!("\t{}: {}", name, balance);
    }
}
