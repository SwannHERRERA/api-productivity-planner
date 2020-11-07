#[derive(Queryable)]
pub struct Objective {
  pub uuid: uuid,
  pub name: String,
  pub start_at: timestamp,
  pub date_created: timestamp,
}

pub struct User {
  pub uuid: uuid,
  pub email: String,
  pub password: String,
  pub date_created: timestamp,
}
