// This file contains code relating to managing deposits.
extern crate chrono;

use chrono::{NaiveDateTime, DateTime, UTC};
use clap::{App, SubCommand, ArgMatches, Arg};
use postgres::{Connection, SslMode};

use currency::Currency;

// Define the data in the database
struct Deposit {
    account: i32,
    at: chrono::NaiveDateTime,
    amount: Currency
}

const ACCOUNT_ID_ARG_NAME:   &'static str = "ACCOUNT_ID";
const BALANCE_ARG_NAME:      &'static str = "AMOUNT";
const DATE_ARG_NAME:         &'static str = "DATE";


// List the deposits made.
fn list_deposits() {
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let stmt = conn.prepare("
        SELECT accounts.id, accounts.name, deposits.amount, deposits.at
        FROM deposits
        INNER JOIN accounts ON deposits.account=accounts.id
        ORDER BY deposits.at ASC"
    ).unwrap();

    println!("Deposits:");

    for row in stmt.query(&[]).unwrap() {
        let account_id: i32 = row.get(0);
        let name: String = row.get(1);
        let amount: Currency = row.get(2);
        let date: chrono::NaiveDateTime = row.get(3);
        println!("\t({}) {}: {} @ {}", account_id, name, amount, date);
    }
}


fn add_deposit(matches: &ArgMatches) {
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let deposit = Deposit {
        account: value_t!(matches.value_of(ACCOUNT_ID_ARG_NAME), i32).unwrap(),
        at: value_t!(matches.value_of(DATE_ARG_NAME), DateTime<UTC>).unwrap().naive_utc(),
        amount: value_t!(matches.value_of(BALANCE_ARG_NAME), Currency).unwrap(),
    };

    let stmt = conn.prepare("
        INSERT INTO deposits (account, at, amount) VALUES
            ($1, $2, $3)
    ").unwrap();
    stmt.execute(&[&deposit.account, &deposit.at, &deposit.amount]).unwrap();
}


// Handle the deposit subcommand.
pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        ("list", Some(_))       => {list_deposits()},
        ("add", Some(matches))  => {add_deposit(matches)},
        _                       => {},
    }
}


pub fn get_subcommands<'a, 'b, 'c, 'd, 'e, 'f>() -> App<'a, 'b, 'c, 'd, 'e, 'f> {
    // Subcommands for the 'deposit' subcommand.
    let deposit_sub = SubCommand::with_name("deposit")
                                 .about("manage deposits");
    let deposit_list = SubCommand::with_name("list")
                                  .about("list all deposits");
    let deposit_add = SubCommand::with_name("add")
                                 .about("add a new deposit")
                                 .arg(
                                      Arg::with_name(ACCOUNT_ID_ARG_NAME)
                                          .help("The account to update")
                                          .required(true)
                                          .index(1)
                                 )
                                 .arg(
                                      Arg::with_name(BALANCE_ARG_NAME)
                                          .help("The amount of the deposit, e.g. £500.00")
                                          .required(true)
                                          .index(2)
                                 )
                                 .arg(
                                      Arg::with_name(DATE_ARG_NAME)
                                          .help("The date the deposit was made, e.g. 2014-11-28T12:00:09Z")
                                          .required(true)
                                          .index(3)
                                 );
    deposit_sub.subcommand(deposit_list).subcommand(deposit_add)
}
