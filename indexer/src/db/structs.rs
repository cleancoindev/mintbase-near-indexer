use super::schema::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde_json::Value;
use std::str::FromStr;

fn string_value_to_date(date_val: &Value) -> NaiveDateTime {
  let date_str = date_val.as_str().unwrap().to_string();
  let date_int = date_str.parse::<i64>().unwrap();
  println!(
    "dat str  {:?}     {:?}",
    date_str,
    NaiveDateTime::from_timestamp(date_int / 1000, 0)
  );
  return NaiveDateTime::from_timestamp(date_int / 1000, 0);
}

pub fn val_to_i16(val: &Value) -> i16 {
  let s = val.as_str().unwrap().to_string();
  return s.parse::<i16>().unwrap();
}

pub fn val_to_i64(val: &Value) -> i64 {
  let s = val.as_str().unwrap().to_string();
  return s.parse::<i64>().unwrap();
}

pub fn val_to_opt_i64(val: &Value) -> Option<i64> {
  return match val.as_str() {
    Some(s) => Some(s.to_string().parse::<i64>().unwrap()),
    _ => None,
  };
}

fn val_vec_to_str(val_vec: &Vec<Value>) -> Vec<String> {
  let mut str_vec: Vec<String> = vec![];

  for val in val_vec {
    str_vec.push(val.as_str().unwrap().to_string());
  }

  return str_vec;
}

fn dai_token() -> u128 {
  let base: u128 = 10;
  return base.pow(18);
}

#[derive(Insertable, Clone, Debug)]
pub struct Store {
  id: String,
  owner: String,
  name: String,
  symbol: String,
  boughtCount: BigDecimal,
  tokenCount: BigDecimal,
  valueCount: BigDecimal,
  transferCount: BigDecimal,
}

impl Store {
  pub fn from_args(args: &Value) -> Self {
    Self {
      id: args["id"].as_str().unwrap().to_string(),
      owner: args["owner"].as_str().unwrap().to_string(),
      name: args["name"].as_str().unwrap().to_string(),
      symbol: args["symbol"].as_str().unwrap().to_string(),
      boughtCount: BigDecimal::from_str("3").unwrap(),
      tokenCount: BigDecimal::from_str("0").unwrap(),
      valueCount: BigDecimal::from_str("0").unwrap(),
      transferCount: BigDecimal::from_str("0").unwrap(),
      // symbol: args["symbol"].as_str().unwrap().to_string(),
    }
  }
}

#[derive(Insertable, Clone, Debug)]
pub struct Thing {
  id: String,
  minter: String,
  timestamp: String,
  burned: bool,
  forSale: bool,
  metaId: String,
}

impl Thing {
  pub fn from_args(args: &Value) -> Self {
    Self {
      id: args["id"].as_str().unwrap().to_string(),
      minter: args["minter"].as_str().unwrap().to_string(),
      timestamp: args["timestamp"].as_str().unwrap().to_string(),
      burned: false,
      forSale: false,
      metaId: args["metaId"].as_str().unwrap().to_string(),
    }
  }
}
