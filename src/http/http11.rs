use crate::http::{Http, KeepAliveState};
use crate::responder::StatusCode;

pub fn http11_check(validator: &mut Http) {
    if validator.request.headers.get("host").is_none() {
        validator.set(StatusCode::BadRequest)
    }
    if let Some(connection_type) = validator.request.headers.get("connection") {
        if connection_type == "keep-alive" {
            validator.set_keepalive(KeepAliveState::UserDefined);
        } else if connection_type == "close" {
            validator.set_keepalive(KeepAliveState::Close)
        }
    }
    // Check for http2 connection header here, if found then call a http2 parse
    // function that will parse http2 frames and parse the request from that
    if let Some(x) = validator.request.headers.get("connection") {
        let value = x.split(',').map(str::to_lowercase).any(|x| x == "upgrade");
        if value {
            if let Some(upgrade_header) = validator.request.headers.get("upgrade") {
                let _values = upgrade_header.split(',');
            }
        }
    }
}
