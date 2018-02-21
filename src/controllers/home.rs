use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use mime;

/// Create a `Handler` that is invoked for requests to the path "/"
pub fn say_hello(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello Router!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

/// Create a `Handler` that is invoked for requests to the path "/hop"
pub fn say_hop(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hop!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

pub fn say_404(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::NotFound,
        Some((String::from("OH NO.").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}