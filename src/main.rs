#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
extern crate rocket;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use dotenv::dotenv;
use diesel::prelude::*;
use rocket_contrib::Template;
use rocket::response::{Redirect, NamedFile};

mod models;
mod schema;
pub mod pooling;

use pooling::{init_pool, DbConn};
use models::{ReqInfo, NewVisit, Visit};
use schema::visits;
use std::path::{PathBuf, Path};

fn add_and_get_visits(conn: DbConn, inf: ReqInfo, msg: &str) -> Template {
	// Make a NewVisit
	let visit = NewVisit {msg: msg, ip: &inf.ip.to_string(), useragent: &inf.useragent};
	
	// Add it to the database
	diesel::insert_into(visits::table)
		.values(&visit)
		.execute(&*conn)
		.expect("Error saving visit");

	// Get all our visits
	let mut results = visits::table
		.load::<Visit>(&*conn)
		.expect("Error loading visits");
	
	results.reverse();
	let mut data = HashMap::new(); 
	data.insert("results", json!(results));
	
	// Display them
	Template::render("index", data)
}

#[get("/")]
fn index(conn: DbConn, inf: ReqInfo) -> Template {
	add_and_get_visits(conn, inf, "Hello, World!")	
}

#[get("/<msg>")]
fn custom_msg(conn: DbConn, inf: ReqInfo, msg: String) -> Template {
	add_and_get_visits(conn, inf, &msg)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/clear")]
fn clear(conn: DbConn) -> Redirect {
	diesel::delete(visits::table)
		.execute(&*conn)
		.expect("Error deleting posts");
	
	Redirect::to("/")
}

fn main() {
	// Get the .env values
	dotenv().ok();

	// Launch rocket
	rocket::ignite()
		.attach(Template::fairing()) // Templating
		.manage(init_pool()) // Manage our database connections
		.mount("/", routes![index, clear, custom_msg])
		.mount("/static/", routes![files])
		.launch();
}
