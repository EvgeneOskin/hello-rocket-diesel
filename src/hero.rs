#[derive(Serialize, Deserialize)]
pub struct Hero {
  pub id: Option<u32>,
  pub name: String,
  pub identity: String,
  pub hometown: String,
  pub age: i32,
}
