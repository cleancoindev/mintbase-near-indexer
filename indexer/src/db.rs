use near_indexer::near_primitives::views::{ExecutionOutcomeWithIdView, ExecutionStatusView};
use near_indexer::Outcome;
use serde_json::{Result, Value};

use bigdecimal::BigDecimal;
use std::str::FromStr;

use diesel::{
  pg::upsert::*,
  prelude::*,
  r2d2::{ConnectionManager, Pool},
};
use tokio_diesel::*;

mod schema;
mod structs;

pub fn continue_if_valid_mintbase_receipt(outcome: Outcome) -> Option<ExecutionOutcomeWithIdView> {
  let receipt: ExecutionOutcomeWithIdView = match outcome {
    Outcome::Receipt(outcome) => outcome,
    _ => return None,
  };

  if receipt.outcome.executor_id != "minthouse.testnet" {
    return None;
  }
  let res: Option<&String> = match &receipt.outcome.status {
    ExecutionStatusView::SuccessValue(res) => Some(res),
    ExecutionStatusView::SuccessReceiptId(receipt_id) => None,
    _ => return None,
  };

  return Some(receipt);
}

pub async fn process_logs(
  pool: &Pool<ConnectionManager<PgConnection>>,
  receipt: ExecutionOutcomeWithIdView,
) -> Result<()> {
  for log in receipt.outcome.logs {
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
  if log_type == &"store_creation".to_string() {
    println!("added store son!!!!");
    add_store(pool, params).await;
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
