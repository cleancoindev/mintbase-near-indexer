use diesel::{
  prelude::*,
  r2d2::{ConnectionManager, Pool},
};
use near_indexer::near_primitives::views::{
  ExecutionOutcomeView, ExecutionOutcomeWithIdView, ExecutionStatusView,
};
use serde_json::{Result, Value};
use tokio_diesel::*;

mod schema;
mod structs;

pub fn check_is_minthouse(account: String) -> bool {
  let haystack: Vec<_> = account.split(".").collect();

  if haystack.contains(&"minthouse") {
    return true;
  } else {
    return false;
  }
}

pub fn continue_if_valid_mintbase_receipt(
  execution_outcome_with_id: ExecutionOutcomeWithIdView,
) -> Option<ExecutionOutcomeView> {
  if check_is_minthouse(execution_outcome_with_id.outcome.executor_id.to_string()) == false {
    return None;
  }

  let res: Option<&String> = match &execution_outcome_with_id.outcome.status {
    ExecutionStatusView::SuccessValue(res) => Some(res),
    ExecutionStatusView::SuccessReceiptId(receipt_id) => None,
    _ => return None,
  };

  return Some(execution_outcome_with_id.outcome);
}

pub async fn process_logs(
  pool: &Pool<ConnectionManager<PgConnection>>,
  outcome: ExecutionOutcomeView,
) -> Result<()> {
  for log in outcome.logs {
    let json: Value = serde_json::from_str(log.as_str())?;
    println!("type: {:?} args: {:?}", &json["type"], &json["params"]);
    execute_log(pool, &json["type"], &json["params"]).await;
  }

  Ok(())
}

pub struct User {
  id: String,
  avitar: String,
}

// TODO: batch tx's
pub async fn execute_log(
  pool: &Pool<ConnectionManager<PgConnection>>,
  log_type: &Value,
  params: &Value,
) {
  println!("log_type -----------------{:?}", log_type);
  if log_type == &"store_creation".to_string() {
    add_store(pool, params).await;
  } else if log_type == &"thing_creation".to_string() {
    add_thing(pool, params).await;
  } else if log_type == &"token_creation".to_string() {
    add_token(pool, params).await;
  } else if log_type == &"store_burned".to_string() {
    burn_store(pool, params).await;
  } else if log_type == &"token_transfered".to_string() {
    transfer_token(pool, params).await;
  } else if log_type == &"burn_token".to_string() {
    burn_token(pool, params).await;
  } else if log_type == &"add_minter".to_string() {
    add_minter(pool, params).await;
  } else if log_type == &"renounce_minter".to_string() {
    renounce_minter(pool, params).await;
  }
}

pub async fn add_store(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let store: structs::Store = structs::Store::from_args(params);

  println!("store to addd!----{:?}", store);

  diesel::insert_into(schema::stores::table)
    .values(store)
    .execute_async(pool)
    .await
    .expect("something went wrong while trying to insert into markets");
}

pub async fn add_thing(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let thing: structs::Thing = structs::Thing::from_args(params);

  diesel::insert_into(schema::things::table)
    .values(thing)
    .execute_async(pool)
    .await
    .expect("something went wrong while trying to insert into markets");
}

pub async fn add_token(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let token: structs::Token = structs::Token::from_args(params);

  diesel::insert_into(schema::tokens::table)
    .values(token)
    .execute_async(pool)
    .await
    .expect("something went wrong while trying to insert into markets");
}

pub async fn add_minter(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let minter: structs::Minter = structs::Minter::from_args(params);

  diesel::insert_into(schema::minters::table)
    .values(minter)
    .execute_async(pool)
    .await
    .expect("something went wrong while trying to insert minter");
}

pub async fn renounce_minter(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let account = params["account"].as_str().unwrap().to_string();

  diesel::update(schema::minters::table.filter(schema::minters::dsl::account.eq(account)))
    .set(schema::minters::dsl::enabled.eq(false))
    .execute_async(pool)
    .await
    .expect("updated renounce minter failed");
}

pub async fn burn_store(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let store_id = params["store_id"].as_str().unwrap().to_string();

  diesel::update(schema::stores::table.filter(schema::stores::dsl::id.eq(store_id)))
    .set(schema::stores::dsl::burned.eq(true))
    .execute_async(pool)
    .await
    .expect("updated store burned failed");
}

pub async fn transfer_token(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let token_key = params["token_key"].as_str().unwrap().to_string();
  let to_account = params["to_account"].as_str().unwrap().to_string();

  diesel::update(schema::tokens::table.filter(schema::tokens::dsl::id.eq(token_key)))
    .set(schema::tokens::dsl::ownerId.eq(to_account))
    .execute_async(pool)
    .await
    .expect("updated store burned failed");
}

pub async fn burn_token(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let token_key = params["token_key"].as_str().unwrap().to_string();

  diesel::update(schema::tokens::table.filter(schema::tokens::dsl::id.eq(token_key)))
    .set(schema::tokens::dsl::burned.eq(true))
    .execute_async(pool)
    .await
    .expect("updated store burned failed");
}

#[test]
fn test_new() {
  let context = check_is_minthouse("contract.minthouse.testnet".to_string());

  assert_eq!(context, true);

  let context2 = check_is_minthouse("minthouse.testnet".to_string());

  assert_eq!(context2, true);

  let context3 = check_is_minthouse("minthouese.testnet".to_string());

  assert_eq!(context3, false);
}
