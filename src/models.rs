use rocket::request::{Request, FromRequest};
use rocket::Outcome;
use rocket::http::Status;
use std::net::SocketAddr;
use schema::visits;

// Visit
#[derive(Queryable)]
pub struct Visit {
	pub id: i32,
	pub ip: String, 
	pub useragent: String,
	pub msg: String
}

#[derive(Insertable)]
#[table_name="visits"]
pub struct NewVisit<'a> {
	pub ip: &'a str, 
	pub useragent: &'a str,
	pub msg: &'a str
}

pub struct ReqInfo {
	pub ip: SocketAddr,
	pub useragent: String 
}

impl<'a, 'r> FromRequest<'a, 'r> for ReqInfo {
	type Error = ();
	fn from_request(req: &'a Request<'r>) -> Outcome<ReqInfo, (Status, ()), ()> {
		Outcome::Success(ReqInfo {ip: req.remote().expect("get request ip"), 
				useragent: req.headers().get_one("User-Agent").expect("get user-agent").to_string()})
	}
}
