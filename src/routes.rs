use gotham::router::Router;
use gotham::router::builder::*;

use controllers;

pub fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(controllers::home::say_hello);
        route.get("/hop").to(controllers::home::say_hop);
        route.get("/404").to(controllers::home::say_404);
        route.scope("/admin", |route| {
            route.get("/404").to(controllers::home::say_404);
            route.get("/hop").to(controllers::home::say_404);
        });
    })
}