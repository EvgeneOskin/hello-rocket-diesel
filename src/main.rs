#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use rocket_contrib::{Json, Value};

mod db;
mod schema;

mod hero;
use hero::{Hero};

#[post("/hero", data="<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
  let insert = Hero {id: None, ..hero.into_inner() };
  Json(Hero::create(insert, &connection))
}

#[get("/hero")]
fn read(connection: db::Connection) -> Json<Value> {
  Json(json!(Hero::read(&connection)))
}

#[put("/hero/<id>", data="<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<Value> {
  let update = Hero { id:Some(id), ..hero.into_inner() };
  Json(json!({
    "success": Hero::update(id, update, &connection)
  }))
}

#[delete("/hero/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
  Json(json!({
    "status": Hero::delete(id, &connection)
  }))
}

fn main() {
    rocket::ignite()
    .manage(db::connect())
    .mount("/", routes![create, read, update, delete])
    .launch();
}
