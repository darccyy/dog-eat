use serde_json::{json, Value::Null};
use unreact::prelude::*;

const URL: &str = "https://darccyy.github.io/dog_eat";

fn main() -> UnreactResult<()> {
  let foods = dog_eat::compile_foods().expect("Could not compile data");
  let categories = dog_eat::sort_categories(&foods);
  let table = dog_eat::make_table(&categories);

  let mut app = Unreact::new(
    Config {
      minify: false,
      ..Config::default()
    },
    is_dev(),
    URL,
  )?;

  app
    // Cannot be concise format
    .set_globals(json!({ "table": table }))
    .index("index", &Null)?
    .not_found("404", &Null)?;

  app.finish()?;

  Ok(())
}
