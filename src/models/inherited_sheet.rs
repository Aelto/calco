use crate::constants::DATABASE_PATH;
use rusqlite::{params, Connection, Result};

#[allow(dead_code)]
pub struct InheritedSheet {
  parent_sheet_id: i32,
  inherited_sheet_id: i32,
  date: i64
}

impl InheritedSheet {
  #[allow(dead_code)]
  pub fn new(parent_sheet_id: i32, inherited_sheet_id: i32, date: i64) -> InheritedSheet {
    InheritedSheet {
      parent_sheet_id,
      inherited_sheet_id,
      date
    }
  }

  #[allow(dead_code)]
  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      insert into inherited_sheets (
        parent_sheet_id,
        inherited_sheet_id,
        date
      )
      values (
        ?1,
        ?2,
        ?3
      )
    ", params![self.parent_sheet_id, self.inherited_sheet_id, self.date])
    .map(|_n| ())
  }

  #[allow(dead_code)]
  pub fn remove(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      delete from inherited_sheets
      where parent_sheet_id = ?1 and inherited_sheet_id = ?2
      ",
      params![self.parent_sheet_id, self.inherited_sheet_id],
    )?;

    Ok(())
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<InheritedSheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select parent_sheet_id, inherited_sheet_id, date
      from inherited_sheets
    ")?;

    let inherited_sheets = query.query_map(params![], |row| {
      Ok(
        InheritedSheet {
          parent_sheet_id: row.get(0)?,
          inherited_sheet_id: row.get(1)?,
          date: row.get(2)?
        }
      )
    })?;

    inherited_sheets.collect()
  }

  #[allow(dead_code)]
  pub fn get_all_by_sheet_id(sheet_id: i32) -> Result<Vec<InheritedSheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select parent_sheet_id, inherited_sheet_id, date
      from inherited_sheets
      where parent_sheet_id = ?1
    ")?;

    let inherited_sheets = query.query_map(params![sheet_id], |row| {
      Ok(
        InheritedSheet {
          parent_sheet_id: row.get(0)?,
          inherited_sheet_id: row.get(1)?,
          date: row.get(2)?
        }
      )
    })?;

    inherited_sheets.collect()
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists inherited_sheets (
      parent_sheet_id integer not null,
      inherited_sheet_id integer not null,
      date datetime not null
    )
  ", params![])
  .map(|_n| ())
}

pub fn remove_all_from_parent_sheet_id(parent_sheet_id: i32) -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    delete from inherited_sheets
    where parent_sheet_id = ?1
    ",
    params![parent_sheet_id],
  )?;

  Ok(())
}

pub fn remove_all_from_inherited_sheet_id(inherited_sheet_id: i32) -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    delete from inherited_sheets
    where inherited_sheet_id = ?1
    ",
    params![inherited_sheet_id],
  )?;

  Ok(())
}