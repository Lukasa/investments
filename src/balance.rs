// This file contains code relating to managing accounts.
extern crate chrono;

use chrono::{NaiveDateTime, UTC};
use clap::{App, SubCommand, ArgMatches, Arg};
use postgres::{Connection, SslMode};

use currency::Currency;

// Define the data in the database.
struct Balance {
    account: i32,
    as_of: chrono::NaiveDateTime,
    balance: Currency
}

const ACCOUNT_ID_ARG_NAME:   &'static str = "ACCOUNT_ID";
const BALANCE_ARG_NAME:      &'static str = "BALANCE";


// List the accounts stored and their balances.
fn list_balances() {
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let stmt = conn.prepare("
        SELECT DISTINCT ON (balance.account) accounts.id, accounts.name, balance.balance
        FROM balance
        INNER JOIN accounts ON balance.account=accounts.id
        ORDER BY balance.account, balance.as_of DESC"
    ).unwrap();

    println!("Balances:");

    for row in stmt.query(&[]).unwrap() {
        let account_id: i32 = row.get(0);
        let name: String = row.get(1);
        let balance: Currency = row.get(2);
        println!("\t({}) {}: {}", account_id, name, balance);
    }
}


fn update_balances(matches: &ArgMatches) {
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let balance = Balance {
        account: value_t!(matches.value_of(ACCOUNT_ID_ARG_NAME), i32).unwrap(),
        as_of: UTC::now().naive_utc(),
        balance: value_t!(matches.value_of(BALANCE_ARG_NAME), Currency).unwrap(),
    };

    let stmt = conn.prepare("
        INSERT INTO balance (account, as_of, balance) VALUES
            ($1, $2, $3)
    ").unwrap();
    stmt.execute(&[&balance.account, &balance.as_of, &balance.balance]).unwrap();
}


// Handle the balance subcommand.
pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        ("list", Some(_))         => {list_balances()},
        ("update", Some(matches)) => {update_balances(matches)},
        _                         => {},
    }
}


pub fn get_subcommands<'a, 'b, 'c, 'd, 'e, 'f>() -> App<'a, 'b, 'c, 'd, 'e, 'f> {
    // Subcommands for the 'balance' subcommand.
    let balances_sub = SubCommand::with_name("balance")
                                  .about("manage balances");
    let balances_list = SubCommand::with_name("list")
                                   .about("list balances by account");
    let balances_update = SubCommand::with_name("update")
                                     .about("add a new balance for an account")
                                     .arg(
                                          Arg::with_name(ACCOUNT_ID_ARG_NAME)
                                              .help("The account to update")
                                              .required(true)
                                              .index(1)
                                     )
                                     .arg(
                                          Arg::with_name(BALANCE_ARG_NAME)
                                              .help("The current balance of the account, e.g. £1234.56")
                                              .required(true)
                                              .index(2)
                                     );
    balances_sub.subcommand(balances_list).subcommand(balances_update)
}
