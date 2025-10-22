mod hasher;

use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

use crate::hasher::hash_util;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeadersRoot) });
}}

struct HttpHeadersRoot;

impl Context for HttpHeadersRoot {}

impl RootContext for HttpHeadersRoot {
    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders { context_id }))
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        info!("Requested on headers for context : {}", self.context_id);
        for (name, value) in &self.get_http_request_headers() {
            info!("#{} -> {}: {}", self.context_id, name, value);
        }

        match self.get_http_request_header(":path") {
            Some(path) if path == "/status" => {
                info!("Got a hello request");
                self.send_http_response(
                    200,
                    vec![("Powered-By", "proxy-wasm")],
                    None,
                );
                Action::Pause
            }
            _ => {
                info!("Did not get a hello request, sending hash token");
                let hash_token = hash_util::authenticate_and_hash( self.get_http_request_header("Authorization"));
                self.set_http_request_header("x-hash-token", hash_token.as_deref());
                Action::Continue
            },
        }
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            info!("#{} <- {}: {}", self.context_id, name, value);
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        info!("#{} completed.", self.context_id);
    }
}