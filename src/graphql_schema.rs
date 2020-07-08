// graphql_schema.rs
use juniper::{EmptyMutation, RootNode};
extern crate dotenv;
use std::env;

#[derive(Queryable)]
pub struct Member {
  pub id: i32,
  pub name: String,
  pub knockouts:i32,
  pub team_id:i32,
} 

#[derive(Queryable)]
pub struct Team {
  pub id: i32,
  pub name: String,
}

#[juniper::object(description = "A member of a team")]
impl Member {
  pub fn id(&self) -> i32 {
    self.id  
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn knockouts(&self) -> i32 {
    self.knockouts
  }

  pub fn team_id(&self) -> i32 {
    self.team
  }
}

#[juniper::object(description = "A team of members")]
impl Team {
  pub fn id(&self) -> i32 {
    self.id
  }

  pub fn name(&self) -> &str {
    self.name.as_str()
  }

  pub fn members(&self) -> Vec<Member> {
    vec![]
  }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
  fn members() -> Vec<Member> {
    vec![
      Member {
        id: 1,
        name: "Link".to_owned(),
      },
      Member {
        id: 2,
        name: "Mario".to_owned(),
      }
    ]
  }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

    pub fn create_schema() -> Schema {
      Schema::new(QueryRoot {}, EmptyMutation::new())
    }