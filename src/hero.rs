use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use schema::heroes;

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Hero {
  pub id: Option<i32>,
  pub name: String,
  pub identity: String,
  pub hometown: String,
  pub age: i32,
}


impl Hero {
  pub fn create(hero: Hero, connection: &SqliteConnection) -> Hero {
    diesel::insert_into(heroes::table)
    .values(&hero)
    .execute(connection)
    .expect("Error creating new hero");

    heroes::table.order(heroes::id.desc()).first(connection).unwrap()
  }

  pub fn read(connection: &SqliteConnection) -> Vec<Hero> {
    heroes::table.order(heroes::id.desc()).load::<Hero>(connection).unwrap()
  }

  pub fn update(id: i32, hero: Hero, connection: &SqliteConnection) -> bool {
    diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
  }

  pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
    diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
  }
}
