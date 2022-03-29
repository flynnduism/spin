use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

wit_bindgen_rust::import!("../../wit/ephemeral/spin-config.wit");

/// A simple Spin HTTP component.
#[http_component]
fn hello_world(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());

    let message = spin_config::get_config("message").expect("config error");

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(message.into()))?)
}
