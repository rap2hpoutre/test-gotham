use gotham::pipeline::new_pipeline;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::single::single_pipeline;
use hyper::StatusCode;

use controllers;
use middlewares;

pub fn router() -> Router {
    build_simple_router(|route| {

        // Handle global 404
        route.add_response_extender(StatusCode::NotFound, controllers::default::NotFoundExtender);

        // Default route
        route.get("/").to(controllers::home::say_hello);

        // 404
        route.get("/404").to(controllers::home::say_404);

        // Admin route
        route.scope("/admin", |route| {
            route.get("/hop").to(controllers::home::say_hop);
        });

        // Sub route with middleware
        route.delegate("/user_agent_terminable_test").to_router({
            let (chain, pipelines) = single_pipeline(new_pipeline().add(middlewares::user_agent::TerminableMiddlewareUserAgent {}).build());
            build_router(chain, pipelines, |route| {
                route.get("/").to(controllers::home::say_hello);
            })
        });
    })
}
