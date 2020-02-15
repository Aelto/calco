use maud::{html, Markup};

pub fn menu() -> Markup {
  html! {
    div.menu {
      h1 { "Calco" }
      a href="/" { "🏠 home" }
      a href="/sheets" { "🧮 sheets" }
    }
  }
}