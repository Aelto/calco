use maud::{html, Markup};

pub fn menu() -> Markup {
  html! {
    div.menu {
      a href="/" { "home" }
    }
  }
}