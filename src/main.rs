extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate log;

mod middlewares {
    pub mod user_agent;
}

mod helpers;
mod routes;
mod controllers {
    pub mod home;
    pub mod default;
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, routes::router())
}