use maud::{html, Markup};

pub fn header(page_title: &str) -> Markup {
  let css_path = format!("/static/{}.css", page_title);

  html! {
    head {
      meta charset="utf-8";
      meta name="viewport" content="width=device-width, initial-scale=1.0";
      meta http-equiv="X-UA-Compatible" content="ie=edge";
  
      link href="/static/master.css" rel="stylesheet";
      link href=(css_path) rel="stylesheet";
  
      title { (page_title) }
    }
  }
}