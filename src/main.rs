#[macro_use(accepts, to_sql_checked)]
extern crate postgres;
extern crate chrono;
extern crate byteorder;
extern crate num;
#[macro_use(value_t)]
extern crate clap;

mod account;
mod balance;
mod currency;
mod deposit;

use clap::{App, ArgMatches};

fn prepare_interface<'a, 'b>() -> ArgMatches<'a, 'b> {
    // Top level command.
    let mut app = App::new("investments")
                      .about("Keeps track of investments");

    // Register top-level subcommands.
    app = app.subcommand(account::get_subcommands());
    app = app.subcommand(balance::get_subcommands());
    app = app.subcommand(deposit::get_subcommands());
    return app.get_matches();
}

fn main() {
    let matches = prepare_interface();

    match matches.subcommand() {
        ("account", Some(matches)) => {account::handle(matches)},
        ("balance", Some(matches)) => {balance::handle(matches)},
        ("deposit", Some(matches)) => {deposit::handle(matches)},
        _                          => {},
    }
}
