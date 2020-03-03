use crate::constants::DATABASE_PATH;
use rusqlite::{params, Connection, Result};

// Represents a source of expense in a `Sheet`
//
//
#[allow(dead_code)]
pub struct Expense {
  pub id: i32,
  pub name: String,
  pub amount: i32,
  pub date: i64,
  pub sheet_id: i32
}

impl Expense {
  #[allow(dead_code)]
  pub fn new(name: &str, amount: i32, sheet_id: i32, date: i64) -> Expense {
    Expense {
      id: 0,
      name: name.to_owned(),
      amount,
      date,
      sheet_id
    }
  }

  #[allow(dead_code)]
  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      insert into expenses (
        name,
        amount,
        date,
        sheet_id
      )
      values (
        ?1,
        ?2,
        ?3,
        ?4
      )
    ", params![self.name, self.amount, self.date, self.sheet_id])
    .map(|_n| ())
  }

  #[allow(dead_code)]
  pub fn remove(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      delete from expenses
      where id = ?1
      ",
      params![self.id],
    )?;

    Ok(())
  }

  #[allow(dead_code)]
  pub fn get_by_name(key: &str) -> Result<Option<Expense>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, date, sheet_id
      from expenses
      where name = ?1
    ")?;

    let mut expenses = query.query_map(params![key], |row| {
      Ok(
        Expense {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          date: row.get(3)?,
          sheet_id: row.get(4)?
        }
      )
    })?;

    expenses.nth(0).transpose()
  }

  #[allow(dead_code)]
  pub fn get_by_id(id: i32) -> Result<Option<Expense>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, date, sheet_id
      from expenses
      where id = ?1
    ")?;

    let mut expenses = query.query_map(params![id], |row| {
      Ok(
        Expense {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          date: row.get(3)?,
          sheet_id: row.get(4)?
        }
      )
    })?;

    expenses.nth(0).transpose()
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<Expense>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, date, sheet_id
      from expenses
    ")?;

    let expenses = query.query_map(params![], |row| {
      Ok(
        Expense {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          date: row.get(3)?,
          sheet_id: row.get(4)?
        }
      )
    })?;

    expenses.collect()
  }

  #[allow(dead_code)]
  pub fn get_all_by_sheet_id(sheet_id: i32) -> Result<Vec<Expense>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, date, sheet_id
      from expenses
      where sheet_id = ?1
    ")?;

    let expenses = query.query_map(params![sheet_id], |row| {
      Ok(
        Expense {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          date: row.get(3)?,
          sheet_id
        }
      )
    })?;

    expenses.collect()
  }

  pub fn update(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      update expenses
      set name = ?1,
          amount = ?2,
          date = ?3
      where id = ?4
      ",
      params![self.name, self.amount, self.date, self.id],
    )?;

    Ok(())
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists expenses (
      id integer primary key autoincrement,
      name text not null,
      amount integer not null,
      date datetime not null,
      sheet_id integer not null
    )
  ", params![])
  .map(|_n| ())
}