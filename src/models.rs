use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Queryable)]
pub struct Objective {
  pub uuid: uuid,
  pub name: String,
  pub start_at: NaiveDateTime,
  pub date_created: NaiveDateTime,
}

pub struct User {
  pub uuid: uuid,
  pub email: String,
  pub password: String,
  pub date_created: NaiveDateTime,
}
