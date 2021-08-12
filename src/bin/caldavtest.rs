extern crate serde;

#[macro_use]
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use reqwest::{header::CONTENT_TYPE, Method};
use yaserde::de::from_str;
use yaserde::YaDeserialize;

// notes if you don't fully parse previous things this breaks easily

#[derive(Debug, YaDeserialize, PartialEq)]
#[yaserde(rename = "d:multistatus")]
struct MultiStatus {
    #[yaserde(rename = "xmlns:d")]
    d: String,

    #[yaserde(rename = "xmlns:cal")]
    cal: String,

    #[yaserde(rename = "xmlns:cs")]
    cs: String,

    #[yaserde(rename = "xmlns:nc")]
    nc: String,

    #[yaserde(rename = "xmlns:oc")]
    oc: String,

    #[yaserde(rename = "xmlns:s")]
    s: String,

    #[yaserde(rename = "$unflatten=response")]
    response: Response,
}

#[derive(Default, Debug, YaDeserialize, PartialEq)]
struct Response {
    #[yaserde(rename = "$unflatten=href")]
    href: String,

    #[yaserde(rename = "$unflatten=propstat")]
    propstat: PropStat,
}

#[derive(Default, Debug, YaDeserialize, PartialEq)]
struct PropStat {
    #[yaserde(rename = "$unflatten=prop")]
    prop: Prop,
    //#[serde(rename = "$unflatten=status")]
    //status: String,
}

#[derive(Default, Debug, YaDeserialize, PartialEq)]
struct Prop {
    #[yaserde(rename = "$unflatten=getctag")]
    getctag: String,
    //#[serde(rename = "$unflatten=resourcetype")]
    //resourcetype: ResourceType,
}

#[derive(Default, Debug, YaDeserialize, PartialEq)]
struct CTag {}

#[derive(Default, Debug, YaDeserialize, PartialEq)]
struct ResourceType {
    #[yaserde(rename = "$unflatten=collection")]
    collection: String,

    #[yaserde(rename = "$unflatten=calendar")]
    calendar: String,
}

#[derive(Default, Debug, YaDeserialize, PartialEq)]
struct Collection {}

#[derive(Default, Debug, YaDeserialize, PartialEq)]
struct Calendar {}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
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

    //let text =
    //  r#"<d:prop><d:resourcetype><d:collection /><cal:calendar /></d:resourcetype><getctag>jj</getctag></d:prop>"#
    //           .to_string();

    println!("{}", text);

    let xml: Prop = from_str(text.as_str())?;

    println!("{:#?}", xml);

    Ok(())
}
