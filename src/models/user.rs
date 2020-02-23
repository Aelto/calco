use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::ops::Add;
use std::option::Option;
use maud::{Render, Markup, html};

use crate::constants;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum UserRole {
  Guest = 0,
  Admin = 100,
  None = -1
}

impl UserRole {
  pub fn from_number(n: i32) -> UserRole {
    match n {
      0 => UserRole::Guest,
      100 => UserRole::Admin,
      _ => UserRole::None,
    }
  }

  #[allow(dead_code)]
  pub fn to_number(&self) -> i32 {
    *self as i32
  }
}

impl Render for UserRole {
  fn render(&self) -> Markup {
    html! {
      @match *self {
        UserRole::Admin => "admin",
        UserRole::Guest => "guest",
        UserRole::None => "visitor"
      }
    }
  }
}

pub struct User {
  pub id: i32,
  pub handle: String,
  pub password: String,
  pub token: String,
  pub token_expire_date: i64,
  pub role: UserRole,
}

impl Render for User {
  fn render(&self) -> Markup {
    html! {
      ul {
        li class="id" { "id: " (self.id) }
        li class="handle" { "handle: " (self.handle) }
        li class="password" { "password: " (self.password) }
        li class="token" { "token: " (self.token) }
        li class="token_expire_date" { "token_expire_date: " (self.token_expire_date) }
        li class="role" { "role: " (self.role) }
      }
    }
  }
}

impl User {
  pub fn new(
    handle: String,
    password: String,
    role: UserRole,
  ) -> User {
    User {
      id: 0,
      handle,
      password,
      token: String::new(),
      token_expire_date: 0,
      role,
    }
  }

  pub fn is_role_greater_or_equal(&self, role: UserRole) -> bool {
    (self.role as i32) >= (role as i32)
  }

  #[allow(dead_code)]
  pub fn delete(&self) -> Result<()> {
    let conn = Connection::open(constants::DATABASE_PATH)?;
  
    conn.execute("
      delete from users
      where id = ?1
    ", params![self.id])?;
  
    Ok(())
  }

  pub fn insert(&self) -> Result<()> {
    let conn = Connection::open(constants::DATABASE_PATH)?;
  
    conn.execute("
      insert into users (
        handle, password, token, token_expire_date, role
      )
      values (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
      )
    ", params![
      self.handle,
      self.password,
      self.token,
      self.token_expire_date,
      self.role as i32
    ])?;
  
    Ok(())
  }

  #[allow(dead_code)]
  pub fn get_all() -> Result<Vec<User>> {
    let conn = Connection::open(constants::DATABASE_PATH)?;

    let mut query = conn.prepare("
      select id, handle, password, token, token_expire_date, role
      from users
    ")?;

    let users = query.query_map(params![], |row| {
      Ok(User {
        id: row.get(0)?,
        handle: row.get(1)?,
        password: row.get(2)?,
        token: row.get(3)?,
        token_expire_date: row.get(4)?,
        role: UserRole::from_number(row.get(5)?),
      })
    })?;

    users.collect()
  }

  pub fn get_by_handle(handle: &str) -> Result<Option<User>> {
    let conn = Connection::open(constants::DATABASE_PATH)?;
  
    let mut query = conn.prepare("
      select id, handle, password, token, token_expire_date, role
      from users
      where handle = ?1
    ")?;
  
    let mut users = query.query_map(params![handle], |row| {
      Ok(User {
        id: row.get(0)?,
        handle: row.get(1)?,
        password: row.get(2)?,
        token: row.get(3)?,
        token_expire_date: row.get(4)?,
        role: UserRole::from_number(row.get(5)?),
      })
    })?;
  
    users.nth(0)
    .transpose()
  }

  pub fn get_by_token(token: &str) -> Result<Option<User>> {
    let conn = Connection::open(constants::DATABASE_PATH)?;
  
    let mut query = conn.prepare("
      select id, handle, password, token, token_expire_date, role
      from users
      where token = ?1
    ")?;
  
    let mut users = query.query_map(params![token], |row| {
      Ok(User {
        id: row.get(0)?,
        handle: row.get(1)?,
        password: row.get(2)?,
        token: row.get(3)?,
        token_expire_date: row.get(4)?,
        role: UserRole::from_number(row.get(5)?),
      })
    })?;
  
    users.nth(0)
    .transpose()
  }
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(constants::DATABASE_PATH)?;

  conn.execute("
    create table if not exists users (
      id integer primary key autoincrement,
      handle text not null,
      password text not null,
      token text not null,
      token_expire_date datetime not null,
      role int not null
    )
  ", params![])?;

  Ok(())
}

pub fn set_user_token(handle: &str, token: &str) -> Result<i64> {
  let conn = Connection::open(constants::DATABASE_PATH)?;
  let expiration_date = new_expiration_date();

  conn.execute("
    update users
    set token = ?1,
        token_expire_date = ?2
    where handle = ?3
  ", params![
    token,
    expiration_date,
    handle
  ])?;

  Ok(expiration_date)
}

pub fn delete_user_by_id(id: i32) -> Result<()> {
  let conn = Connection::open(constants::DATABASE_PATH)?;

  conn.execute("
    delete from users
    where id = ?1
  ", params![id])?;

  Ok(())
}

fn new_expiration_date() -> i64 {
  SystemTime::now()
      .add(Duration::new(constants::USER_TOKEN_EXPIRE_DURATION_SECONDS, 0))
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs() as i64
}