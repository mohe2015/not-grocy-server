extern crate quick_xml;
extern crate serde;
use quick_xml::de::from_str;
use reqwest::{header::CONTENT_TYPE, Method};
use serde::Deserialize;

// notes if you don't fully parse previous things this breaks easily

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "d:multistatus")]
struct MultiStatus {
    #[serde(rename = "xmlns:d")]
    d: String,

    #[serde(rename = "xmlns:cal")]
    cal: String,

    #[serde(rename = "xmlns:cs")]
    cs: String,

    #[serde(rename = "xmlns:nc")]
    nc: String,

    #[serde(rename = "xmlns:oc")]
    oc: String,

    #[serde(rename = "xmlns:s")]
    s: String,

    #[serde(rename = "$unflatten=response")]
    response: Response,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Response {
    #[serde(rename = "$unflatten=href")]
    href: String,

    #[serde(rename = "$unflatten=propstat")]
    propstat: PropStat,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PropStat {
    #[serde(rename = "$unflatten=prop")]
    prop: Prop,
    //#[serde(rename = "$unflatten=status")]
    //status: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Prop {
    #[serde(rename = "$unflatten=getctag")]
    getctag: String,
    //#[serde(rename = "$unflatten=resourcetype")]
    //resourcetype: ResourceType,
}

#[derive(Debug, Deserialize, PartialEq)]
struct CTag {}

#[derive(Debug, Deserialize, PartialEq)]
struct ResourceType {
    #[serde(rename = "$unflatten=collection")]
    collection: String,

    #[serde(rename = "$unflatten=calendar")]
    calendar: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Collection {}

#[derive(Debug, Deserialize, PartialEq)]
struct Calendar {}

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

    let mut text = response.text().await?;

    text =
       r#"<d:prop><d:resourcetype><d:collection /><cal:calendar /></d:resourcetype><getctag>jj</getctag></d:prop>"#
                .to_string();

    println!("{}", text);

    let xml: Prop = from_str(text.as_str())?;

    println!("{:#?}", xml);

    Ok(())
}
