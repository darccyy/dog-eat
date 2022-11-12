use serde::Serialize;
use std::fs;

/// Food object
#[derive(Debug, Serialize, Clone)]
pub struct Food {
  pub name: String,
  pub category: Category,
  pub extra: String,
  pub tags: Vec<String>,
  pub info: String,
  pub review: String,
}

/// Category enum for safety of food
#[derive(Debug, Serialize, Clone)]
pub enum Category {
  Fine,
  Limit,
  Bad,
}

/// Object of all categories of food
#[derive(Debug, Serialize)]
pub struct Categories<'a> {
  pub fine: Vec<&'a Food>,
  pub limit: Vec<&'a Food>,
  pub bad: Vec<&'a Food>,
}

/// Convert text file of data to vector of food object
pub fn compile_foods() -> Result<Vec<Food>, Box<dyn std::error::Error>> {
  let mut foods = Vec::new();

  let file = fs::read_to_string("./data.txt")?;

  for (i, cat) in file.split("===").map(|x| x.trim()).enumerate() {
    let entries: Vec<&str> = cat.split("---").map(|x| x.trim()).collect();

    for entry in entries {
      let mut lines = entry.lines();

      if let Some(name) = lines.next() {
        foods.push(Food {
          name: name.to_string(),

          category: match i {
            0 => Category::Fine,
            1 => Category::Limit,
            2 => Category::Bad,
            //TODO Handle!
            _ => panic!("Unknown category! For section {i}"),
          },

          extra: lines.next().unwrap_or("").to_string(),

          tags: lines
            .next()
            .unwrap_or("")
            .to_string()
            .split(",")
            .map(|x| x.trim().to_string())
            .collect(),
          info: lines.next().unwrap_or("").to_string(),

          review: lines.next().unwrap_or("").to_string(),
        });
      }
    }
  }

  Ok(foods)
}

/// Sort vector of food objects into categories
pub fn sort_categories(foods: &Vec<Food>) -> Categories {
  let mut cats = Categories {
    fine: Vec::new(),
    limit: Vec::new(),
    bad: Vec::new(),
  };

  for food in foods {
    match food.category {
      Category::Fine => cats.fine.push(food),
      Category::Limit => cats.limit.push(food),
      Category::Bad => cats.bad.push(food),
    }
  }

  cats
}

/// Convert list of categories to table format for template
pub fn make_table<'a>(cats: &'a Categories) -> Vec<[Option<&'a &'a Food>; 3]> {
  let mut rows = Vec::new();

  let longest = *[cats.fine.len(), cats.limit.len(), cats.bad.len()]
    .iter()
    .reduce(|accum, item| std::cmp::max(accum, item))
    .unwrap_or(&0);

  for i in 0..longest {
    rows.push([cats.fine.get(i), cats.limit.get(i), cats.bad.get(i)]);
  }

  rows
}
