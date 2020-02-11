use crate::constants;
use crate::models::user::{User, UserRole};
use crate::utils::crypto::hash_password;
use rusqlite::{params, Connection, Result};
use std::ops::Add;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use maud::{Render, html, Markup};

pub struct Invitation {
  pub id: i32,
  pub hash: String,
  pub handle: String,
  pub expire_date: i64,
  pub user_role: UserRole,
}

impl Render for Invitation {
  fn render(&self) -> Markup {
    html! {
      ul {
        li class="id" { "id: " (self.id) }
        li class="handle" { "handle: " (self.handle) }
        li class="user_role" { "user_role: " (self.user_role) }
        li class="hash" { "hash: " (self.hash) }
        li class="expire_date" { "expire_date: " (self.expire_date) }
      }
    }
  }
}

impl Invitation {
  pub fn new(handle: &str, user_role: UserRole) -> Invitation {
    let expire_date = new_expiration_date();

    Invitation {
      hash: Uuid::new_v4().to_string(),
      expire_date,
      handle: handle.to_owned(),
      id: 0,
      user_role,
    }
  }

  pub fn insert(invitation: &Invitation) -> Result<()> {
    let conn = Connection::open(constants::DATABASE_PATH)?;

    conn.execute(
      "
      insert into invitations (
        handle, hash, expire_date, user_role
      )
      values (
        ?1,
        ?2,
        ?3,
        ?4
      )
    ",
      params![
        invitation.handle,
        invitation.hash,
        invitation.expire_date,
        invitation.user_role as i32
      ],
    )?;
    Ok(())
  }

  pub fn get_by_hash(hash: &str) -> Result<Option<Invitation>> {
    let conn = Connection::open(constants::DATABASE_PATH)?;

    let mut query = conn.prepare(
      "
      select id, handle, hash, expire_date, user_role
      from invitations
      where hash = ?1
    ",
    )?;
    let mut invitations = query.query_map(params![hash], |row| {
      Ok(Invitation {
        id: row.get(0)?,
        handle: row.get(1)?,
        hash: row.get(2)?,
        expire_date: row.get(3)?,
        user_role: UserRole::from_number(row.get(4)?),
      })
    })?;

    invitations.nth(0).transpose()
  }

  pub fn get_all() -> Result<Vec<Invitation>> {
    let conn = Connection::open(constants::DATABASE_PATH)?;

    let mut query = conn.prepare(
      "
      select id, handle, hash, expire_date, user_role
      from invitations
    ",
    )?;

    let invitations = query.query_map(params![], |row| {
      Ok(Invitation {
        id: row.get(0)?,
        handle: row.get(1)?,
        hash: row.get(2)?,
        expire_date: row.get(3)?,
        user_role: UserRole::from_number(row.get(4)?),
      })
    })?;

    invitations.collect()
  }

  pub fn get_by_handle(handle: &str) -> Result<Option<Invitation>> {
    let conn = Connection::open(constants::DATABASE_PATH)?;

    let mut query = conn.prepare(
      "
      select id, handle, hash, expire_date, user_role
      from invitations
      where handle = ?1
    ",
    )?;
    let mut invitations = query.query_map(params![handle], |row| {
      Ok(Invitation {
        id: row.get(0)?,
        handle: row.get(1)?,
        hash: row.get(2)?,
        expire_date: row.get(3)?,
        user_role: UserRole::from_number(row.get(4)?),
      })
    })?;

    invitations.nth(0).transpose()
  }

  pub fn remove(invitation: &Invitation) -> Result<()> {
    let conn = Connection::open(constants::DATABASE_PATH)?;

    conn.execute(
      "
      delete from invitations
      where id = ?1
    ",
      params![invitation.id],
    )?;
    Ok(())
  }

  pub fn to_url(self: &Invitation) -> String {
    format!("/signup?hash={}&handle={}", self.hash, self.handle)
  }

  pub fn consume(self: &Invitation, password: String) -> std::result::Result<(), String> {
    let hashed_password = hash_password(&self.handle, &password)
      .map_err(|err| format!("error when hashing password {}", err))?;

    let user = User::new(self.handle.clone(), hashed_password, self.user_role);

    user.insert().map_err(|err| format!("error when inserting user {}", err))?;
    Invitation::remove(&self).map_err(|err| format!("error when removing invitation {}", err))?;

    Ok(())
  }
}

fn new_expiration_date() -> i64 {
  SystemTime::now()
    .add(Duration::new(
      constants::INVITATION_EXPIRE_DURATION_SECONDS,
      0,
    ))
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs() as i64
}

pub fn create_table() -> Result<()> {
  let conn = Connection::open(constants::DATABASE_PATH)?;

  conn.execute(
    "
    create table if not exists invitations (
      id integer primary key autoincrement,
      handle text not null,
      hash text not null,
      expire_date datetime not null,
      user_role int not null
    )
  ",
    params![],
  )?;

  Ok(())
}

pub fn update_invitation_expiration(handle: &str) -> Result<()> {
  let conn = Connection::open(constants::DATABASE_PATH)?;
  let expire_date = new_expiration_date();

  conn.execute(
    "
    update invitations
    set expire_date = ?1
    where handle = ?2
  ",
    params![expire_date, handle],
  )?;

  Ok(())
}