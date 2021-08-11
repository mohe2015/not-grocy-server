extern crate quick_xml;
extern crate serde;
use quick_xml::de::from_str;
use reqwest::{header::CONTENT_TYPE, Method};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "d:multistatus")]
struct MultiStatus {}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://github.com/marshalshi/caldav-client-rust
    // https://marshalshi.medium.com/rust-caldav-client-from-scratch-da173cfc905d
    // https://sabre.io/dav/building-a-caldav-client/
    // probably no caldav: https://github.com/fmeringdal/nettu-schedulerc

    let body = "";

    // https://cloud.selfmade4u.de/remote.php/dav/calendars/Moritz.Hedtke/not-grocy/
    let url = std::env::var("URL").expect("URL required");
    let password = std::env::var("PASSWORD").expect("PASSWORD required");
    let client = reqwest::Client::new();
    let response = client
        .request(Method::from_bytes(b"PROPFIND").expect("PROPFIND"), url)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth("Moritz.Hedtke", Some(password))
        .body(body)
        .send()
        .await?;

    let text = response.text().await?;

    println!("{}", text);

    let xml: MultiStatus = from_str(text.as_str())?;

    println!("{:?}", xml);

    Ok(())
}
