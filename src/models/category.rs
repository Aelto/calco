use crate::constants::DATABASE_PATH;
use rusqlite::{params, Connection, Result};

// Represents a source of expense in a `Sheet`
//
//
pub struct Category {
  pub id: i32,
  pub name: String,
  pub sheet_id: i32
}

impl Category {
  #[allow(dead_code)]
  pub fn new(name: &str, sheet_id: i32) -> Category {
    Category {
      id: 0,
      name: name.to_owned(),
      sheet_id
    }
  }

  #[allow(dead_code)]
  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      insert into categories (
        name,
        sheet_id
      )
      values (
        ?1,
        ?2,
      )
    ", params![self.name, self.sheet_id])
    .map(|_n| ())
  }

  #[allow(dead_code)]
  pub fn get_by_name(key: &str) -> Result<Option<Category>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, sheet_id
      from categories
      where name = ?1
    ")?;

    let mut categories = query.query_map(params![key], |row| {
      Ok(
        Category {
          id: row.get(0)?,
          name: row.get(1)?,
          sheet_id: row.get(2)?
        }
      )
    })?;

    categories.nth(0).transpose()
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<Category>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, sheet_id
      from categories
    ")?;

    let categories = query.query_map(params![], |row| {
      Ok(
        Category {
          id: row.get(0)?,
          name: row.get(1)?,
          sheet_id: row.get(2)?
        }
      )
    })?;

    categories.collect()
  }

  #[allow(dead_code)]
  pub fn get_all_by_sheet_id(sheet_id: i32) -> Result<Vec<Category>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, sheet_id
      from categories
      where sheet_id = ?
    ")?;

    let categories = query.query_map(params![sheet_id], |row| {
      Ok(
        Category {
          id: row.get(0)?,
          name: row.get(1)?,
          sheet_id
        }
      )
    })?;

    categories.collect()
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists categories (
      id integer primary key autoincrement,
      name text not null,
      sheet_id integer not null
    )
  ", params![])
  .map(|_n| ())
}