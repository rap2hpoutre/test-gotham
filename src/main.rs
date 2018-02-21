extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;

mod routes;
mod controllers {
    pub mod home;
}

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, routes::router())
}