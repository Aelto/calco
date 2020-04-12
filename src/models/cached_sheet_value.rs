use crate::constants::DATABASE_PATH;
use rusqlite::{params, Connection, Result};

#[allow(dead_code)]
pub struct CachedSheetValue {
  pub sheet_id: i32,
  pub value: i64
}

impl CachedSheetValue {
  #[allow(dead_code)]
  pub fn new(sheet_id: i32, value: i64) -> CachedSheetValue {
    CachedSheetValue {
      sheet_id,
      value
    }
  }

  #[allow(dead_code)]
  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      insert into inherited_sheets (
        sheet_id,
        value
      )
      values (
        ?1,
        ?2
      )
    ", params![self.sheet_id, self.value])
    .map(|_n| ())
  }

  #[allow(dead_code)]
  pub fn update(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      update cached_sheet_values
      set value = ?1
      where sheet_id = ?2
      ",
      params![self.value, self.sheet_id],
    )?;

    Ok(())
  }

  #[allow(dead_code)]
  pub fn remove(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      delete from cached_sheet_values
      where sheet_id = ?1
      ",
      params![self.sheet_id],
    )?;

    Ok(())
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<CachedSheetValue>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select sheet_id, value
      from cached_sheet_values
    ")?;

    let inherited_sheets = query.query_map(params![], |row| {
      Ok(
        CachedSheetValue {
          sheet_id: row.get(0)?,
          value: row.get(1)?
        }
      )
    })?;

    inherited_sheets.collect()
  }

  #[allow(dead_code)]
  pub fn get_by_sheet_id(sheet_id: i32) -> Result<Option<CachedSheetValue>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select sheet_id, value
      from cached_sheet_values
      where sheet_id = ?1
    ")?;

    let mut inherited_sheets = query.query_map(params![sheet_id], |row| {
      Ok(
        CachedSheetValue {
          sheet_id: row.get(0)?,
          value: row.get(1)?
        }
      )
    })?;

    inherited_sheets.nth(0).transpose()
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists cached_sheet_values (
      sheet_id integer not null,
      value integer not null
    )
  ", params![])
  .map(|_n| ())
}

pub fn remove_by_sheet_id(sheet_id: i32) -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    delete from cached_sheet_values
    where sheet_id = ?1
    ",
    params![sheet_id],
  )?;

  Ok(())
}