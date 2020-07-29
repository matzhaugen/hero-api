#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
mod db;
mod schema;

use rocket_contrib::json::{Json, JsonValue};

mod hero;
use hero::{Hero};

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> JsonValue {
    json!(Hero::read(&connection))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>) -> Json<Hero> {
	let update = Hero { id: Some(id), ..hero.into_inner() };
    json!({
        "success": Hero::update(id, update, &connection)
    })
}

#[delete("/<id>")]
fn delete(id: i32) -> JsonValue {
    json!({
        "success": Hero::delete(id, &connection)
    })
}

fn main() {
    rocket::ignite()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .manage(db::connect())
        .launch();
}