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
  pub name: String,
  pub cached_value: i32
}

impl Sheet {
  #[allow(dead_code)]
  pub fn new(name: &str) -> Sheet {
    Sheet {
      id: 0,
      cached_value: 0,
      name: name.to_owned()
    }
  }

  #[allow(dead_code)]
  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(DATABASE_PATH)?;

    conn.execute("
      insert into sheets (
        name,
        cached_value
      )
      values (
        ?1,
        ?2
      )
    ", params![self.name, self.cached_value])
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
      set name = ?1,
          cached_value = ?2
      where id = ?3
      ",
      params![self.name, self.cached_value, self.id],
    )?;

    Ok(())
  }

  #[allow(dead_code)]
  pub fn get_by_name(key: &str) -> Result<Option<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, cached_value
      from sheets
      where name = ?1
    ")?;

    let mut configs = query.query_map(params![key], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?,
          cached_value: row.get(2)?
        }
      )
    })?;

    configs.nth(0).transpose()
  }

  pub fn get_by_id(id: i32) -> Result<Option<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, cached_value
      from sheets
      where id = ?1
    ")?;

    let mut configs = query.query_map(params![id], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?,
          cached_value: row.get(2)?
        }
      )
    })?;

    configs.nth(0).transpose()
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, cached_value
      from sheets
    ")?;

    let sheets = query.query_map(params![], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?,
          cached_value: row.get(2)?
        }
      )
    })?;

    sheets.collect()
  }

  #[allow(dead_code)]
  pub fn get_all_sheets_by_parent_sheet_id(sheet_id: i32) -> Result<Vec<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, cached_value
      from sheets
      join inherited_sheets on inherited_sheet_id = id
      where parent_sheet_id = ?1
    ")?;

    let sheets = query.query_map(params![sheet_id], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?,
          cached_value: row.get(2)?
        }
      )
    })?;

    sheets.collect()
  }

  pub fn get_all_sheets_by_inherited_sheet_id(inherited_sheet_id: i32) -> Result<Vec<Sheet>> {
    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, name, cached_value
      from sheets
      join inherited_sheets on parent_sheet_id = id
      where inherited_sheet_id = ?1
    ")?;

    let sheets = query.query_map(params![inherited_sheet_id], |row| {
      Ok(
        Sheet {
          id: row.get(0)?,
          name: row.get(1)?,
          cached_value: row.get(2)?
        }
      )
    })?;

    sheets.collect()
  }

  pub fn add_to_cached_value(&mut self, value: i32) -> Result<()> {
    self.cached_value += value;

    self.update()?;

    Sheet::update_inheriting_sheets(self.id, value)
  }

  pub fn remove_from_cached_value(&mut self, value: i32) -> Result<()> {
    self.cached_value -= value;

    self.update()?;

    Sheet::update_inheriting_sheets(self.id, -value)
  }

  pub fn update_inheriting_sheets(first_sheet_id: i32, change: i32) -> Result<()> {
    use std::collections::VecDeque;

    let mut sheets_to_update: VecDeque<Sheet> = VecDeque::new();

    println!("starting nested updating sequence, change : {}", change);

    for sheet in Sheet::get_all_sheets_by_inherited_sheet_id(first_sheet_id)? {
      println!("adding sheet {} to update queue", sheet.name);

      sheets_to_update.push_back(sheet);
    }

    let mut current_sheet = sheets_to_update.pop_front();

    let conn = Connection::open(DATABASE_PATH)?;

    let mut query = conn.prepare("
      update sheets
      set cached_value = cached_value + ?1
      where id = ?2
    ")?;

    while current_sheet.is_some() {
      if let Some(sheet) = current_sheet {
        println!("updating sheet {}", sheet.name);

        query.execute(params![change, sheet.id])?;

        for child_sheet in Sheet::get_all_sheets_by_inherited_sheet_id(sheet.id)? {
          println!("adding sheet {} to update queue", child_sheet.name);
          
          sheets_to_update.push_back(child_sheet);
        }
      }

      current_sheet = sheets_to_update.pop_front();
    }

    Ok(())
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(DATABASE_PATH)?;

  conn.execute("
    create table if not exists sheets (
      id integer primary key autoincrement,
      name text not null,
      cached_value integer not null
    )
  ", params![])
  .map(|_n| ())
}