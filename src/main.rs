#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

use dotenv::dotenv;

mod models;
mod schema;
pub mod pooling;
use pooling::{init_pool, DbConn};
use models::{ReqInfo, NewVisit, Visit};
use schema::visits;
use diesel::prelude::*;

fn add_and_get_visits(conn: DbConn, inf: ReqInfo, msg: &str) -> String {
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

	// Display them
	let mut out = format!("Visits: {}\n", results.len());
	for post in results {
		out = format!("{}{} {} {}\n", out, post.msg, post.ip, post.useragent);
	}
	out
}

#[get("/")]
fn index(conn: DbConn, inf: ReqInfo) -> String {
	add_and_get_visits(conn, inf, "Hello, World!")	
}

#[get("/<msg>")]
fn custom_msg(conn: DbConn, inf: ReqInfo, msg: String) -> String {
	add_and_get_visits(conn, inf, &msg)
}

fn main() {
	// Get the .env values
	dotenv().ok();

	// Launch rocket
	rocket::ignite()
		.manage(init_pool()) // Manage our database connections
		.mount("/", routes![index, custom_msg])
		.launch();
}
