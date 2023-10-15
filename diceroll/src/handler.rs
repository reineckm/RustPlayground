use std::fs;

use crate::{Context, Response};
use hyper::StatusCode;
use serde::Deserialize;

mod simul;

pub async fn roll_handler(_ctx: Context) -> String {
    return format!("{}", simul::dice_roll());
}

pub async fn main_handler(_ctx: Context) -> String {    
    let data = fs::read_to_string("./static/index.html").expect("Should have been able to read the file");
    return data;
}

#[derive(Deserialize)]
struct SendRequest {
    name: String,
    active: bool,
}

pub async fn send_handler(mut ctx: Context) -> Response {
    let body: SendRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    Response::new(
        format!(
            "send called with name: {} and active: {}",
            body.name, body.active
        )
        .into(),
    )
}

pub async fn param_handler(ctx: Context) -> String {
    let param = match ctx.params.find("some_param") {
        Some(v) => v,
        None => "empty",
    };
    format!("param called, param was: {}", param)
}