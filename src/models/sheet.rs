use crate::constants::DATABASE_PATH;
use crate::models::inherited_sheet;
use crate::models::cached_sheet_value;
use rusqlite::{params, Connection, Result};

/// Represents a calculus sheet,
/// it could be a month, a week or a day.
/// Everything is tied to a sheet in the end.
#[allow(dead_code)]
pub struct Sheet {
  pub id: i32,
  pub name: String
}

impl Sheet {
  #[allow(dead_code)]
  pub fn new(name: &str) -> Sheet {
    Sheet {
      id: 0,
      name: name.to_owned()
    }
  }

  #[allow(dead_code)]
  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      insert into sheets (
        name
      )
      values (
        ?1
      )
    ", params![self.name])
    .map(|_n| ())
  }

  #[allow(dead_code)]
  pub fn remove(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      delete from sheets
      where id = ?1
      ",
      params![self.id],
    )?;

    inherited_sheet::remove_all_from_inherited_sheet_id(self.id)?;
    inherited_sheet::remove_all_from_parent_sheet_id(self.id)?;
    cached_sheet_value::remove_by_sheet_id(self.id)?;

    Ok(())
  }

  pub fn update(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      update sheets
      set name = ?1
      where id = ?2
      ",
      params![self.name, self.id],
    )?;

    Ok(())
  }

  #[allow(dead_code)]
  pub fn get_by_name(key: &str) -> Result<Option<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name
      from sheets
      where name = ?1
    ")?;

    let mut configs = query.query_map(params![key], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?
        }
      )
    })?;

    configs.nth(0).transpose()
  }

  pub fn get_by_id(id: i32) -> Result<Option<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name
      from sheets
      where id = ?1
    ")?;

    let mut configs = query.query_map(params![id], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?
        }
      )
    })?;

    configs.nth(0).transpose()
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name
      from sheets
    ")?;

    let sheets = query.query_map(params![], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?
        }
      )
    })?;

    sheets.collect()
  }

  #[allow(dead_code)]
  pub fn get_all_sheets_by_parent_sheet_id(sheet_id: i32) -> Result<Vec<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name
      from sheets
      join inherited_sheets on inherited_sheet_id = id
      where parent_sheet_id = ?1
    ")?;

    let sheets = query.query_map(params![sheet_id], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?
        }
      )
    })?;

    sheets.collect()
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists sheets (
      id integer primary key autoincrement,
      name text not null
    )
  ", params![])
  .map(|_n| ())
}