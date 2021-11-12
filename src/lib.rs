#![cfg(target_arch = "wasm32")]

use proxy_wasm as wasm;
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::{Action, LogLevel};

use log::info;

#[no_mangle]
pub fn _start() {
    wasm::set_log_level(LogLevel::Trace);

    // Note: there are also RootContext and StreamContext that provide different callbacks
    wasm::set_http_context(
        |context_id, root_context_id| -> Box<dyn HttpContext> {
            Box::new(MyContext {
                context_id,
                root_context_id,
            })
        }
    );
}

struct MyContext {
    context_id: u32,
    root_context_id: u32,
}

impl MyContext {}

impl Context for MyContext {
    fn on_done(&mut self) -> bool {
        true
    }
}

impl HttpContext for MyContext {
    fn on_http_request_headers(&mut self, _num_headers: usize) -> Action {
        info!("Request from WASM");
        Action::Continue
    }

    fn on_http_request_body(&mut self, _body_size: usize, _stream_end: bool) -> Action {
        Action::Continue
    }

    fn on_http_request_trailers(&mut self, _num_trailers: usize) -> Action {
        Action::Continue
    }

    fn on_http_response_headers(&mut self, _num_headers: usize) -> Action {
        info!("Response from WASM");
        self.add_http_response_header("x-my-custom-header", "hello world");
        Action::Continue
    }

    fn on_http_response_body(&mut self, _body_size: usize, _stream_end: bool) -> Action {
        Action::Continue
    }

    fn on_http_response_trailers(&mut self, _num_trailers: usize) -> Action {
        Action::Continue
    }
}