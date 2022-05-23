use super::schema::ethbalances;

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use chrono::NaiveDateTime;

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::*;
use diesel::{backend::Backend, expression::AsExpression};
use ethers::prelude::{Address as EthereumAddress, U256 as Eth256, *};
use serde::Serialize;
use std::io::Write;

#[derive(Queryable, Debug)]
pub struct GetETHRioBalance {
    pub id: i32,
    pub account: Address,
    pub balance: U256,
    pub holder: bool,
    pub last_updated: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "ethbalances"]
pub struct PostETHRioBalance {
    pub id: i32,
    pub account: Address,
    pub balance: U256,
    pub holder: bool,
    pub last_updated: NaiveDateTime,
}

// Diesel/ Rust Type Reference:  https://gist.github.com/steveh/7c7145409a5eed6b698ee8b609b6d1fc
#[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Serialize)]
#[sql_type = "Varchar"]
pub struct Address {
    value: EthereumAddress,
}

impl ToSql<VarChar, Pg> for Address {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <String as ToSql<VarChar, Pg>>::to_sql(&self.value.to_string(), out)
    }
}

impl<DB: Backend<RawValue = [u8]>> FromSql<Varchar, DB> for Address {
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        <String as FromSql<VarChar, Pg>>::from_sql(bytes).map(|s| Address {
            value: s.parse().unwrap(),
        })
    }
}

#[derive(AsExpression, FromSqlRow, Debug, Copy, Clone, Serialize)]
#[sql_type = "Numeric"]
pub struct U256 {
    value: Eth256,
}

impl ToSql<Numeric, Pg> for U256 {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        <BigDecimal as ToSql<Numeric, Pg>>::to_sql(
            &BigDecimal::from_u128(self.value.as_u128()).unwrap(),
            out,
        )
    }
}

impl<DB: Backend<RawValue = [u8]>> FromSql<U256, DB> for U256 {
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        <BigDecimal as FromSql<Numeric, Pg>>::from_sql(bytes).map(|s| U256 {
            value: Eth256::from(s.to_u128().unwrap()),
        })
    }
}
