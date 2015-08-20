// This file contains code relating to managing accounts.
use clap::{App, SubCommand, ArgMatches, Arg};
use postgres::{Connection, SslMode};

const ACCOUNT_NAME_ARG_NAME: &'static str = "ACCOUNT_NAME";
const ACCOUNT_TYPE_ARG_NAME: &'static str = "ACCOUNT_TYPE";

// An account object that matches the database.
struct Account {
    id: i32,
    name: String,
    kind: String,
}

fn add_account(matches: &ArgMatches) {
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let account = Account {
        id: 0,
        name: matches.value_of(ACCOUNT_NAME_ARG_NAME).unwrap().to_string(),
        kind: matches.value_of(ACCOUNT_TYPE_ARG_NAME).unwrap().to_string(),
    };

    let stmt = conn.prepare("
        INSERT INTO accounts (name, kind) VALUES ($1, $2)
    ").unwrap();
    let updates = stmt.execute(&[&account.name, &account.kind]).unwrap();

    if updates == 0 {println!("Failed to add new account.")}
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
pub fn handle(matches: &ArgMatches) {
    match matches.subcommand() {
        ("list", Some(matches)) => {list_accounts()},
        ("add", Some(matches))  => {add_account(matches)},
        _                       => {},
    }
}

pub fn get_subcommands<'a, 'b, 'c, 'd, 'e, 'f>() -> App<'a, 'b, 'c, 'd, 'e, 'f> {
    // Subcommands for the 'account' subcommand.
    let accounts_sub = SubCommand::with_name("account")
                                  .about("manage accounts");
    let accounts_list = SubCommand::with_name("list")
                                   .about("list accounts");
    let accounts_add = SubCommand::with_name("add")
                                  .about("add new account")
                                  .arg(
                                      Arg::with_name(ACCOUNT_NAME_ARG_NAME)
                                          .help("The name of the new account")
                                          .required(true)
                                          .index(1)
                                  )
                                  .arg(
                                      Arg::with_name(ACCOUNT_TYPE_ARG_NAME)
                                          .help("The type of the new account")
                                          .required(true)
                                          .index(2)
                                  );
    accounts_sub.subcommand(accounts_list).subcommand(accounts_add)
}