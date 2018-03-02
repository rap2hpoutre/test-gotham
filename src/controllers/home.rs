use gotham::state::State;
use hyper::Response;
use helpers;


pub fn say_hello(state: State) -> (State, Response) {
    helpers::response_text("Hello Router!", state)
}

/// Create a `Handler` that is invoked for requests to the path "/hop"
pub fn say_hop(state: State) -> (State, Response) {
    helpers::response_text("Hop!", state)
}

pub fn say_404(state: State) -> (State, Response) {
    helpers::response_not_found_text("Oh no", state)
}