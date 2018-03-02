use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use mime;

fn response_text_with_status(text: &str, state: State, status: StatusCode) -> (State, Response) {
    let res = create_response(
        &state,
        status,
        Some((String::from(text).into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}

pub fn response_text(text: &str, state: State) -> (State, Response) {
    response_text_with_status(text, state, StatusCode::Ok)
}

pub fn response_not_found_text(text: &str, state: State) -> (State, Response) {
    response_text_with_status(text, state, StatusCode::NotFound)
}