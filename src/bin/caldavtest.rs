use reqwest::{header::CONTENT_TYPE, Method, Result};

#[actix_web::main]
async fn main() -> Result<()> {
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

    Ok(())
}
