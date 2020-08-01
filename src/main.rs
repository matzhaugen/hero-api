#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;

use rocket;
use rocket::{post, get, put, delete, routes};
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::{database, json};

#[database("psql_logs")]
struct LogsDbConn(diesel::PgConnection);

mod schema;

mod hero;
use hero::{Hero};

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: LogsDbConn) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &*connection))
}

#[get("/")]
fn read(connection: LogsDbConn) -> JsonValue {
    json!(Hero::read(&*connection))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: LogsDbConn) -> JsonValue {
	let update = Hero { id: Some(id), ..hero.into_inner() };
    json!({
        "success": Hero::update(id, update, &*connection)
    })
}

#[delete("/<id>")]
fn delete(id: i32, connection: LogsDbConn) -> JsonValue {
    json!({
        "success": Hero::delete(id, &*connection)
    })
}

fn main() {
    rocket::ignite()
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .attach(LogsDbConn::fairing())
        .launch();
}