use hyper::{Response, StatusCode};
use gotham::state::State;
use gotham::router::response::extender::ResponseExtender;
use gotham::http::response::extend_response;
use mime;

pub struct NotFoundExtender;
impl ResponseExtender for NotFoundExtender {
    fn extend(&self, state: &mut State, response: &mut Response) {
        extend_response(
            state,
            response,
            StatusCode::NotFound,
            Some((b"404 Not Found".to_vec(), mime::TEXT_PLAIN))
        );
    }
}