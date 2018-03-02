use futures::{future, Future};
use hyper::header::{Headers, UserAgent};
use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::{FromState, State};

#[derive(StateData)]
pub struct TerminableMiddlewareUserAgentData {
    pub user_agent: String,
    pub supported: bool,
}

#[derive(Clone, NewMiddleware)]
pub struct TerminableMiddlewareUserAgent;

impl Middleware for TerminableMiddlewareUserAgent {
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        let user_agent = {
            let headers = Headers::borrow_from(&state);
            match headers.get::<UserAgent>() {
                Some(ua) => ua.to_string(),
                None => String::from("None"),
            }
        };

        state.put(TerminableMiddlewareUserAgentData {
            user_agent,
            supported: false,
        });

        let result = chain(state);

        let f = result.and_then(move |(state, mut response)| {
            {
                let headers = response.headers_mut();
                let data = TerminableMiddlewareUserAgentData::borrow_from(&state);
                println!(
                    "Supplied: {}, Supported: {}",
                    data.user_agent, data.supported
                );
                headers.set_raw(
                    "X-User-Agent",
                    format!(
                        "Supplied: {}, Supported: {}",
                        data.user_agent, data.supported
                    ),
                );
            };
            future::ok((state, response))
        });

        Box::new(f)
    }
}
