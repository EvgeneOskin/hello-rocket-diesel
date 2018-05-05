#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::{Json, Value};

mod hero;
use hero::{Hero};

#[post("/hero", data="<hero>")]
fn create(hero: Json<Hero>) -> Json<Hero> {
  hero
}

#[get("/hero")]
fn read() -> Json<Value> {
  Json(json!([
    "hero 1",
    "hero 2",
  ]))
}

#[put("/hero/<id>", data="<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
  hero
}

#[delete("/hero/<id>")]
fn delete(id: i32) -> Json<Value> {
  Json(json!({"status": "ok"}))
}

fn main() {
    rocket::ignite()
    .mount("/", routes![create, read, update, delete])
    .launch();
}
