extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;

use crate::db::PgPool;
use crate::schema::recipes;


#[derive(Clone)]
pub struct Context {
  pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
  fn recipes(context: &Context) -> Vec<Recipe> {
    use crate::schema::recipes::dsl::*;
    let connection = context.db.get().unwrap();;
    recipes
      .limit(10)
      .load::<Recipe>(&connection)
      .expect("Error loading recipes")
  }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
  fn create_recipe(context: &Context, data: NewRecipe) -> Recipe {
    let connection = context.db.get().unwrap();;
    diesel::insert_into(recipes::table)
      .values(&data)
      .get_result(&connection)
      .expect("Error saving new post")
  }
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "recipes"]
pub struct NewRecipe {
  pub id: i32,
  pub name: String,
  pub author: String,
  pub description: String,
  pub ingredients: Vec<String>,
  pub method: Vec<String>,
  pub url: String,
}


#[derive(Queryable)]
pub struct Recipe {
  pub id: i32,
  pub name: String,
  pub author: String,
  pub description: String,
  pub ingredients: Vec<String>,
  pub method: Vec<String>,
  pub url: String,
}

#[juniper::object(Context = Context, description = "A recipe")]
impl Recipe {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn author(&self) -> &str {
    self.author.as_str()
  }

  pub fn description(&self) -> &str {
    self. description.as_str()
  }

  pub fn ingredients(&self) -> Vec<&str> {
    self.ingredients.iter().map(|s| &**s).collect()
  }

  pub fn method(&self) -> Vec<&str> {
    self.method.iter().map(|s| &**s).collect()
  }

  pub fn url(&self) -> &str {
    self.url.as_str()
  }
}



pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
