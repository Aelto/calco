use crate::constants::DATABASE_PATH;
use rusqlite::{params, Connection, Result};

use crate::models::sheet::Sheet;

// Represents a source of expense in a `Sheet`
//
//
pub struct Expense {
  pub id: i32,
  pub name: String,
  pub amount: i32,
  pub sheet_id: i32,

  pub sheet: Option<Sheet>
}

impl Expense {
  pub fn new(name: &str, amount: i32, sheet_id: i32) -> Expense {
    Expense {
      id: 0,
      name: name.to_owned(),
      amount,
      sheet_id,
      sheet: None
    }
  }

  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      insert into expenses (
        name,
        amount,
        sheet_id
      )
      values (
        ?1,
        ?2,
        ?3
      )
    ", params![self.name, self.amount, self.sheet_id])
    .map(|_n| ())
  }

  pub fn get_by_name(key: &str) -> Result<Option<Expense>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, sheet_id
      from expenses
      where name = ?1
    ")?;

    let mut expenses = query.query_map(params![key], |row| {
      Ok(
        Expense {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          sheet_id: row.get(3)?,
          sheet: None
        }
      )
    })?;

    expenses.nth(0).transpose()
  }

  pub fn get_all() -> Result<Vec<Expense>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, sheet_id
      from expenses
    ")?;

    let expenses = query.query_map(params![], |row| {
      Ok(
        Expense {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          sheet_id: row.get(3)?,
          sheet: None
        }
      )
    })?;

    expenses.collect()
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists expenses (
      id integer primary key autoincrement,
      name text not null,
      amount integer not null,
      sheet_id integer not null
    )
  ", params![])
  .map(|_n| ())
}