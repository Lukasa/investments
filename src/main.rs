#[macro_use(accepts, to_sql_checked)]
extern crate postgres;
extern crate chrono;
extern crate byteorder;
extern crate num;
#[macro_use(value_t)]
extern crate clap;

mod currency;

use postgres::{Connection, SslMode};
use chrono::{NaiveDateTime, UTC};
use currency::Currency;
use clap::{App, SubCommand, ArgMatches, Arg};

const ACCOUNT_ID_ARG_NAME: &'static str = "ACCOUNT_ID";
const BALANCE_ARG_NAME:    &'static str = "BALANCE";

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

fn prepare_interface<'a, 'b>() -> ArgMatches<'a, 'b> {
    // Top level command.
    let mut app = App::new("investments")
                      .about("Keeps track of investments");

    // Subcommands for the 'account' subcommand.
    let mut accounts_sub = SubCommand::with_name("account")
                                  .about("manage accounts");
    let accounts_list = SubCommand::with_name("list")
                                   .about("list accounts");
    accounts_sub = accounts_sub.subcommand(accounts_list);

    // Subcommands for the 'balance' subcommand.
    let mut balances_sub = SubCommand::with_name("balance")
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
    balances_sub = balances_sub.subcommand(balances_list)
                               .subcommand(balances_update);

    // Register top-level subcommands.
    app = app.subcommand(accounts_sub);
    app = app.subcommand(balances_sub);

    return app.get_matches();
}


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
        id: 0,
        account: value_t!(matches.value_of(ACCOUNT_ID_ARG_NAME), i32).unwrap(),
        as_of: UTC::now().naive_utc(),
        balance: value_t!(matches.value_of(BALANCE_ARG_NAME), Currency).unwrap(),
    };

    let stmt = conn.prepare("
        INSERT INTO balance (account, as_of, balance) VALUES
            ($1, $2, $3)
    ").unwrap();
    let updates = stmt.execute(&[&balance.account, &balance.as_of, &balance.balance]).unwrap();
}


// Show the accounts in the system.
fn list_accounts() {
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let stmt = conn.prepare("
        SELECT accounts.id, accounts.name
        FROM accounts
        ORDER BY accounts.id"
    ).unwrap();

    println!("Accounts:");

    for row in stmt.query(&[]).unwrap() {
        let account_id: i32 = row.get(0);
        let name: String = row.get(1);
        println!("\t{}: {}", account_id, name);
    }

}


// Handle the account subcommand.
fn handle_accounts(matches: &ArgMatches) {
    match matches.subcommand() {
        ("list", Some(matches)) => {list_accounts()},
        _                       => {},
    }
}


// Handle the balance subcommand.
fn handle_balances(matches: &ArgMatches) {
    match matches.subcommand() {
        ("list", Some(matches))   => {list_balances()},
        ("update", Some(matches)) => {update_balances(matches)},
        _                         => {},
    }
}

fn main() {
    let matches = prepare_interface();

    match matches.subcommand() {
        ("account", Some(matches)) => {handle_accounts(matches)},
        ("balance", Some(matches)) => {handle_balances(matches)},
        _                          => {},
    }
}
