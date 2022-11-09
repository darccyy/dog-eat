use serde_json::{json, Value};
use unreact::prelude::*;

// Where the site is hosted
const URL: &str = "https://darccyy.github.io/unreact-template";

fn main() -> UnreactResult<()> {
  let mut app = Unreact::new(Config::default(), is_dev(), URL)?;

  app
    // Index page
    .index("index", &json!({"secret": "Hello!"}))?
    // 404 page
    .not_found("404", &Value::Null)?;

  // Custom page
  app.page("hello/there", "hello", &Value::Null)?;

  // Complete app
  app.finish()?;

  Ok(())
}
