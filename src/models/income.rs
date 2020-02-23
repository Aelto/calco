use crate::constants::DATABASE_PATH;
use rusqlite::{params, Connection, Result};

// Represents a source of income in a `Sheet`
//
//
#[allow(dead_code)]
pub struct Income {
  pub id: i32,
  pub name: String,
  pub amount: i32,
  pub date: i64,
  pub sheet_id: i32
}

impl Income {
  #[allow(dead_code)]
  pub fn new(name: &str, amount: i32, sheet_id: i32, date: i64) -> Income {
    Income {
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
      insert into incomes (
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
      delete from incomes
      where id = ?1
      ",
      params![self.id],
    )?;

    Ok(())
  }
  
  #[allow(dead_code)]
  pub fn get_by_name(key: &str) -> Result<Option<Income>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, date, sheet_id
      from incomes
      where name = ?1
    ")?;

    let mut incomes = query.query_map(params![key], |row| {
      Ok(
        Income {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          date: row.get(3)?,
          sheet_id: row.get(4)?
        }
      )
    })?;

    incomes.nth(0).transpose()
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<Income>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, date, sheet_id
      from incomes
    ")?;

    let incomes = query.query_map(params![], |row| {
      Ok(
        Income {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          date: row.get(3)?,
          sheet_id: row.get(4)?
        }
      )
    })?;

    incomes.collect()
  }

  #[allow(dead_code)]
  pub fn get_all_by_sheet_id(sheet_id: i32) -> Result<Vec<Income>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, amount, date, sheet_id
      from incomes
      where sheet_id = ?1
    ")?;

    let incomes = query.query_map(params![sheet_id], |row| {
      Ok(
        Income {
          id: row.get(0)?,
          name: row.get(1)?,
          amount: row.get(2)?,
          date: row.get(3)?,
          sheet_id
        }
      )
    })?;

    incomes.collect()
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists incomes (
      id integer primary key autoincrement,
      name text not null,
      amount integer not null,
      date datetime not null,
      sheet_id integer not null
    )
  ", params![])
  .map(|_n| ())
}