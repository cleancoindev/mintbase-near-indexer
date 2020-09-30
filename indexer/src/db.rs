use near_indexer::near_primitives::views::{
  ExecutionOutcomeView, ExecutionOutcomeWithIdView, ExecutionStatusView,
};
// use near_indexer::ExecutionOutcomeWithIdView;
use serde_json::{Result, Value};

use diesel::{
  prelude::*,
  r2d2::{ConnectionManager, Pool},
};
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

// TODO: batch tx's
pub async fn execute_log(
  pool: &Pool<ConnectionManager<PgConnection>>,
  log_type: &Value,
  params: &Value,
) {
  println!("log_type -----------------{:?}", log_type);
  if log_type == &"store_creation".to_string() {
    println!("added store son!!!!");
    add_store(pool, params).await;
  } else if log_type == &"thing_creation".to_string() {
    println!("added store son!!!!");
    add_thing(pool, params).await;
    add_token(pool, params).await;
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

pub async fn add_user(pool: &Pool<ConnectionManager<PgConnection>>, params: &Value) {
  let user: structs::User = structs::User::from_args(params);

  diesel::insert_into(schema::users::table)
    .values(user)
    .execute_async(pool)
    .await
    .expect("something went wrong while trying to insert into markets");
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
