use super::schema::*;
use bigdecimal::BigDecimal;
use serde_json::Value;
use std::str::FromStr;

#[allow(non_snake_case)]
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
      owner: args["owner_id"].as_str().unwrap().to_string(),
      name: args["name"].as_str().unwrap().to_string(),
      symbol: args["symbol"].as_str().unwrap().to_string(),
      boughtCount: BigDecimal::from_str("3").unwrap(),
      tokenCount: BigDecimal::from_str("0").unwrap(),
      valueCount: BigDecimal::from_str("0").unwrap(),
      transferCount: BigDecimal::from_str("0").unwrap(),
    }
  }
}

#[allow(non_snake_case)]
#[derive(Insertable, Clone, Debug)]
pub struct Thing {
  id: String,
  minter: String,
  burned: bool,
  forSale: bool,
  metaId: String,
  resolveStore: String,
}

impl Thing {
  pub fn from_args(args: &Value) -> Self {
    Self {
      id: args["id"].as_str().unwrap().to_string(),
      minter: args["minter"].as_str().unwrap().to_string(),
      burned: false,
      forSale: false,
      metaId: args["meta_id"].as_str().unwrap().to_string(),
      resolveStore: args["store"].as_str().unwrap().to_string(),
    }
  }
}

#[allow(non_snake_case)]
#[derive(Insertable, Clone, Debug)]
pub struct Token {
  id: String,
  tokenId: String,
  metaId: String,
  price: String,
  burned: bool,
  state: String,
  transferCount: BigDecimal,
  storeId: String,
  ownerId: String,
}

impl Token {
  pub fn from_args(args: &Value) -> Self {
    Self {
      id: args["id"].as_str().unwrap().to_string(),
      tokenId: args["token_id"].as_str().unwrap().to_string(),
      metaId: args["meta_id"].as_str().unwrap().to_string(),
      price: args["price"].as_str().unwrap().to_string(),
      burned: false,
      state: args["state"].as_str().unwrap().to_string(),
      transferCount: BigDecimal::from_str("1").unwrap(),
      storeId: args["store"].as_str().unwrap().to_string(),
      ownerId: args["minter"].as_str().unwrap().to_string(),
    }
  }
}

#[allow(non_snake_case)]
#[derive(Insertable, Clone, Debug)]
pub struct Minter {
  account: String,
  store: String,
  enabled: bool,
}

impl Minter {
  pub fn from_args(args: &Value) -> Self {
    Self {
      account: args["account"].as_str().unwrap().to_string(),
      store: args["store"].as_str().unwrap().to_string(),
      enabled: true,
    }
  }
}

#[derive(Insertable, Clone, Debug)]
pub struct User {
  id: String,
  avitar: String,
}

impl User {
  pub fn from_args(args: &Value) -> Self {
    Self {
      id: args["id"].as_str().unwrap().to_string(),
      avitar: "any".to_string(),
    }
  }
}

#[allow(non_snake_case)]
#[derive(Insertable, Clone, Debug)]
pub struct Indexer {
  network: String,
  syncedBlock: String,
}

impl Indexer {
  pub fn from_args(args: &Value) -> Self {
    Self {
      network: args["network"].as_str().unwrap().to_string(),
      syncedBlock: args["syncedBlock"].as_str().unwrap().to_string(),
    }
  }
}
