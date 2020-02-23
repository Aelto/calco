use rusqlite::{Result};

pub mod sheet;
pub mod income;
pub mod expense;
pub mod category;
pub mod user;
pub mod invitation;
pub mod inherited_sheet;

pub fn create_database() -> Result<()> {
  println!("creating database tables");

  sheet::create_table()
  .and(user::create_table())
  .and(category::create_table())
  .and(income::create_table())
  .and(expense::create_table())
  .and(invitation::create_table())
  .and(inherited_sheet::create_table())
  .and(create_admin_invitation())
}

pub fn create_admin_invitation() -> Result<()> {
  use crate::constants;
  use crate::models::invitation::{Invitation, update_invitation_expiration};
  use crate::models::user::{UserRole};
  use clipboard::{ClipboardProvider, ClipboardContext};

  let invitation = Invitation::get_by_handle(constants::ADMIN_HANDLE)?
  .or_else(|| {
    let new_invitation = Invitation::new(
      constants::ADMIN_HANDLE,
      UserRole::Admin
    );

    Invitation::insert(&new_invitation)
    .expect("error when inserting new admin invitation");

    Some(new_invitation)
  });

  match invitation {
    Some(inv) => {
      use std::io::prelude::Write;

      update_invitation_expiration(constants::ADMIN_HANDLE)?;

      println!(
        "an admin invitation was made for {}, use it to create an account or delete it later on", 
        constants::ADMIN_HANDLE
      );

      if let Ok(context) = ClipboardProvider::new() {
        let mut context: ClipboardContext = context;
        if let Err(err) = context.set_contents(inv.to_url()) {
          println!("could not set clipboard content {}", err);
        }
      }
      else {
        println!("could not write invitation link to clipboard");
      }

      println!("{}", inv.to_url());

      if let Err(e) = std::fs::File::create("invitation.txt")
      .and_then(|mut file| file.write_all(inv.to_url().as_bytes())) {
        println!("an error occured when writing invitation to file {}", e);
      };
    }
    None => println!("could not create admin invitation")
  }

  Ok(())
}