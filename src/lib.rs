#![cfg(target_arch = "wasm32")]

use proxy_wasm as wasm;
use proxy_wasm::traits::{Context, HttpContext};
use proxy_wasm::types::{Action, LogLevel};

#[no_mangle]
pub fn _start() {
    wasm::set_log_level(LogLevel::Trace);

    // Note: there are also RootContext and StreamContext that provide different callbacks
    wasm::set_http_context(
        |context_id, root_context_id| -> Box<dyn HttpContext> {
            Box::new(MyContext {
                context_id: context_id,
                root_context_id: root_context_id,
            })
        }
    );
}

struct MyContext {
    context_id: u32,
    root_context_id: u32,
}

impl MyContext {
}

impl Context for MyContext {
    fn on_done(&mut self) -> bool {
        true
    }
}

impl HttpContext for MyContext {
    fn on_http_request_headers(&mut self, num_headers: usize) -> Action {
        Action::Continue
    }

    fn on_http_request_trailers(&mut self, num_trailers: usize) -> Action {
        Action::Continue
    }

    fn on_http_request_body(&mut self, body_size: usize, stream_end: bool) -> Action {
        Action::Continue
    }

    fn on_http_response_headers(&mut self, num_headers: usize) -> Action {
        self.add_http_response_header("x-my-custom-header", "hello world");
        Action::Continue
    }

    fn on_http_response_trailers(&mut self, num_trailers: usize) -> Action {
        Action::Continue
    }

    fn on_http_response_body(&mut self, body_size: usize, stream_end: bool) -> Action {
        Action::Continue
    }
}