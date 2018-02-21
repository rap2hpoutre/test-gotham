//! An introduction to fundamental `Router` and `Router Builder` concepts to create a routing tree.
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;

mod routes;

mod controllers {
    pub mod home;
}


/// Create a `Router`
///
/// Provides tree of routes with only a single top level entry that looks like:
///
/// /                     --> GET
///
/// If no match for a request is found a 404 will be returned. Both the HTTP verb and the request
/// path are considered when determining if the request matches a defined route.


/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);

    // All incoming requests are delegated to the router for further analysis and dispatch
    gotham::start(addr, routes::router())
}