extern crate serde;

extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use reqwest::{header::CONTENT_TYPE, Method};
use yaserde::de::from_str;

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "d",
    rename = "multistatus",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct MultiStatus {
    #[yaserde(prefix = "d", rename = "response")]
    response: Response,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct Response {
    #[yaserde(prefix = "d", rename = "href")]
    href: String,

    #[yaserde(prefix = "d", rename = "propstat")]
    propstat: PropStat,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct PropStat {
    #[yaserde(prefix = "d", rename = "prop")]
    prop: Prop,

    #[yaserde(prefix = "d", rename = "status")]
    status: String,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns",
    namespace = "x1: http://apple.com/ns/ical/",
    namespace = "x2: http://nextcloud.com/ns"
)]
struct Prop {
    #[yaserde(prefix = "d", rename = "resourcetype")]
    resourcetype: Option<ResourceType>,

    #[yaserde(prefix = "cs", rename = "getctag")]
    getctag: Option<String>,

    #[yaserde(prefix = "s", rename = "sync-token")]
    sync_token: Option<i32>,

    #[yaserde(prefix = "cal", rename = "supported-calendar-component-set")]
    supported_calendar_component_set: Option<SupportedCalendarComponentSet>,

    #[yaserde(prefix = "cal", rename = "schedule-calendar-transp")]
    schedule_calendar_transp: Option<ScheduleCalendarTransp>,

    #[yaserde(prefix = "oc", rename = "owner-principal")]
    owner_principal: Option<String>,

    #[yaserde(prefix = "d", rename = "displayname")]
    displayname: Option<String>,

    #[yaserde(prefix = "cal", rename = "calendar-timezone")]
    calendar_timezone: Option<String>,

    #[yaserde(prefix = "x1", rename = "calendar-order")]
    calendar_order: Option<String>,

    #[yaserde(prefix = "x1", rename = "calendar-color")]
    calendar_color: Option<String>,

    #[yaserde(prefix = "x2", rename = "owner-displayname")]
    owner_displayname: Option<String>,

    #[yaserde(prefix = "d", rename = "current-user-principal")]
    current_user_principal: Option<CurrentUserPrincipal>,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CurrentUserPrincipal {}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct SupportedCalendarComponentSet {
    #[yaserde(prefix = "cal", rename = "comp")]
    comp: Component,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct Component {
    #[yaserde(attribute)]
    name: String,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct ScheduleCalendarTransp {
    #[yaserde(prefix = "cal", rename = "opaque")]
    opaque: String,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct ResourceType {
    #[yaserde(prefix = "d", rename = "collection")]
    collection: Collection,

    #[yaserde(prefix = "cal", rename = "calendar")]
    calendar: Calendar,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct Collection {}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct Calendar {}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "d",
    rename = "propfind",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct Propfind {
    #[yaserde(prefix = "d", rename = "prop")]
    prop: Prop,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // https://github.com/marshalshi/caldav-client-rust
    // https://marshalshi.medium.com/rust-caldav-client-from-scratch-da173cfc905d
    // https://sabre.io/dav/building-a-caldav-client/

    let body_xml = Propfind {
        prop: Prop {
            current_user_principal: Some(CurrentUserPrincipal {}),
            ..Default::default()
        },
    };

    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };

    let body = yaserde::ser::to_string_with_config(&body_xml, &yaserde_cfg).unwrap();

    println!("{}", body);

    // https://cloud.selfmade4u.de/remote.php/dav/calendars/Moritz.Hedtke/not-grocy/
    let url = std::env::var("URL").expect("URL required");
    let password = std::env::var("PASSWORD").expect("PASSWORD required");
    let client = reqwest::Client::new();
    let response = client
        .request(Method::from_bytes(b"PROPFIND").expect("PROPFIND"), url)
        .header("Depth", 0)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth("Moritz.Hedtke", Some(password))
        .body(body)
        .send()
        .await?;

    let text = response.text().await?;

    println!("{}", text);

    let xml: MultiStatus = from_str(text.as_str())?;

    println!("{:#?}", xml);

    Ok(())
}
