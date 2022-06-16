
use esp_idf_svc::httpd as idf;
use std::sync::{Condvar, Mutex};
use anyhow::bail;
use std::{sync::Arc};
use embedded_svc::httpd::registry::*;
use embedded_svc::httpd::*;

use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "data/public/"]
#[prefix = "public/"]
struct Asset;

#[allow(unused_variables)]
pub fn web_server(mutex: Arc<(Mutex<Option<u32>>, Condvar)>) -> Result<idf::Server> {
    let server = idf::ServerRegistry::new()
        .at("/")
        .get(|_| {
            Response::new(200)
            .body(Body::from(std::str::from_utf8(Asset::get("public/index.html").unwrap().data.as_ref()).unwrap()))
            .into()
        })?
        .at("/foo")
        .get(|_| bail!("Boo, something happened!"))?
        .at("/bar")
        .get(|_| {
            Response::new(403)
                .status_message("No permissions")
                .body("You have no permissions to access this page".into())
                .into()
        })?
        .at("/panic")
        .get(|_| panic!("User requested a panic!"))?;

    server.start(&Default::default())
}
