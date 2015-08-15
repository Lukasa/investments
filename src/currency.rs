use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use postgres::types::{FromSql, ToSql, Type, SessionInfo, IsNull};
use postgres::error::Error;
use std::io::{Read, Write};
use std::ops::{Add, Sub};
use std::fmt;
use num::integer::{div_rem};

// This is a really stupid currency type. We'll improve this later.
// It mainly exists to provide an implementation of the MONEY type in Postgres.
#[derive(Eq, Ord, PartialEq, PartialOrd, Clone, Debug)]
pub struct Currency(i64);

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

// TODO: Have a currency display that understands locale.
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (top, rem) = div_rem(self.0, 100);
        write!(f, "£{}.{:02}", top, rem)
    }
}

impl ::std::str::FromStr for Currency {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Currency, &'static str> {
        let mut result: i64 = 0;
        let mut chars: Vec<char> = s.chars().collect();

        // Remove currency symbol.
        chars.remove(0);


        for c in chars {
            match c {
                '0' ... '9' => {result = (result * 10) + (c as i64 - '0' as i64)}
                '.' | ',' | ' ' => {}  // Ignore punctuation
                _ => {return Err("invalid character")}
            }
        }

        Ok(Currency(result))
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


// Tests for Currency
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn currency_from_basic_string() {
        let test_string = "£123,456.78";
        let parsed: Currency = test_string.parse().unwrap();
        assert_eq!(parsed, Currency(12345678));
    }
}
