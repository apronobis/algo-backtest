use serde::{
    de::{Error, Unexpected},
    Deserialize,
    Deserializer,
};

use crate::ty::{OrdType, Side, Status, TimeInForce};

mod rest;
mod stream;

pub use rest::*;
pub use stream::*;

fn from_str_to_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f32>().map_err(Error::custom)
}

fn from_str_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(Error::custom)
}

fn from_str_to_f32_opt<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    match s {
        Some(s) => Ok(Some(s.parse::<f32>().map_err(Error::custom)?)),
        None => Ok(None),
    }
}

fn from_str_to_side<'de, D>(deserializer: D) -> Result<Side, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "BUY" => Ok(Side::Buy),
        "SELL" => Ok(Side::Sell),
        s => Err(Error::invalid_value(Unexpected::Other(s), &"BUY or SELL")),
    }
}

fn from_str_to_status<'de, D>(deserializer: D) -> Result<Status, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "NEW" => Ok(Status::New),
        "PARTIALLY_FILLED" => Ok(Status::PartiallyFilled),
        "FILLED" => Ok(Status::Filled),
        "CANCELED" => Ok(Status::Canceled),
        // "REJECTED" => Ok(Status::Rejected),
        "EXPIRED" => Ok(Status::Expired),
        // "EXPIRED_IN_MATCH" => Ok(Status::ExpiredInMatch),
        s => Err(Error::invalid_value(
            Unexpected::Other(s),
            &"NEW,PARTIALLY_FILLED,FILLED,CANCELED,EXPIRED",
        )),
    }
}

fn from_str_to_type<'de, D>(deserializer: D) -> Result<OrdType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "LIMIT" => Ok(OrdType::Limit),
        "MARKET" => Ok(OrdType::Market),
        // "STOP" => Ok(OrdType::StopLimit),
        // "TAKE_PROFIT" => Ok(OrdType::TakeProfitLimit),
        // "STOP_MARKET" => Ok(OrdType::StopMarket),
        // "TAKE_PROFIT_MARKET" => Ok(OrdType::TakeProfitMarket),
        // "TRAILING_STOP_MARKET" => Ok(OrdType::TrailingStopMarket),
        s => Err(Error::invalid_value(Unexpected::Other(s), &"LIMIT,MARKET")),
    }
}

fn from_str_to_tif<'de, D>(deserializer: D) -> Result<TimeInForce, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    match s {
        "GTC" => Ok(TimeInForce::GTC),
        "IOC" => Ok(TimeInForce::IOC),
        "FOK" => Ok(TimeInForce::FOK),
        "GTX" => Ok(TimeInForce::GTX),
        // "GTD" => Ok(TimeInForce::GTD),
        s => Err(Error::invalid_value(
            Unexpected::Other(s),
            &"GTC,IOC,FOK,GTX",
        )),
    }
}
