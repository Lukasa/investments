#[macro_use(accepts, to_sql_checked)]
extern crate postgres;
extern crate chrono;
extern crate byteorder;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use std::io::{Read, Write};
use std::ops::{Add, Sub};
use std::fmt;

use postgres::{Connection, SslMode};
use postgres::types::{FromSql, ToSql, Type, SessionInfo, IsNull};
use postgres::error::Error;
use chrono::NaiveDateTime;

// This is a really stupid currency type. We'll improve this later.
// It mainly exists to provide an implementation of the MONEY type in Postgres.
#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
struct Currency(i64);

impl Add for Currency {
    type Output = Currency;

    fn add(self, other: Currency) -> Currency {
        Currency(self.0 + other.0)
    }
}

impl Sub for Currency {
    type Output = Currency;

    fn sub(self, other: Currency) -> Currency {
        Currency(self.0 - other.0)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql for Currency {
    fn from_sql<R: Read>(_: &Type, raw: &mut R, _: &SessionInfo) -> Result<Currency, Error> {
        // Money is sent simply as an int64.
        let t = try!(raw.read_i64::<BigEndian>());
        Ok(Currency(t))
    }

    accepts!(Type::Money);
}

impl ToSql for Currency {
    fn to_sql<W: Write+?Sized>(&self, _: &Type, mut w: &mut W, _: &SessionInfo) -> Result<IsNull, Error> {
        let Currency(val) = *self;
        try!(w.write_i64::<BigEndian>(val));
        Ok(IsNull::No)
    }

    accepts!(Type::Money);
    to_sql_checked!();
}

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
    println!("Hello, world!");
    let conn = Connection::connect("postgres://cory@localhost:5432/finances", &SslMode::None).unwrap();

    let stmt = conn.prepare("SELECT * FROM balance").unwrap();
    for row in stmt.query(&[]).unwrap() {
        let balance = Balance{
            id: row.get(0),
            account: row.get(1),
            as_of: row.get(2),
            balance: row.get(3)
        };
        println!("Found balance: {} at {}", balance.balance, balance.as_of);
    }
}
