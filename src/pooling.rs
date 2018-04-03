extern crate r2d2;
extern crate r2d2_diesel;
extern crate diesel;
extern crate rocket;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use std::ops::Deref;
use std::env;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
	let url = env::var("DATABASE_URL")
		.expect("DATABASE_URL is not set!");
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::new(manager).expect("db pool")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
