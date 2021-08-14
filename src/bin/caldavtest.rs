extern crate serde;

extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::process::exit;

use reqwest::{header::CONTENT_TYPE, Method};
use url::Url;
use yaserde::de::from_str;

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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
    response: Vec<Response>,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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
    propstat: Vec<PropStat>,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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

    #[yaserde(prefix = "d", rename = "getetag")]
    getetag: Option<String>,

    #[yaserde(prefix = "cal", rename = "calendar-data")]
    calendar_data: Option<String>,

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

    #[yaserde(prefix = "cal", rename = "calendar-home-set")]
    calendar_home_set: Option<CalendarHomeSet>,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CurrentUserPrincipal {
    #[yaserde(prefix = "d", rename = "href")]
    href: Option<String>,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CalendarHomeSet {
    #[yaserde(prefix = "d", rename = "href")]
    href: Option<String>,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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
    comp: Vec<Component>,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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
    collection: Option<Collection>,

    #[yaserde(prefix = "cal", rename = "calendar")]
    calendar: Option<Calendar>,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct Collection {}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct Calendar {}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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
    #[yaserde(prefix = "d", rename = "self")]
    the_self: Option<TheSelf>,

    #[yaserde(prefix = "d", rename = "prop")]
    prop: Prop,
}

#[derive(serde::Deserialize, Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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
struct TheSelf {}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // DONE https://github.com/marshalshi/caldav-client-rust
    // DONE https://marshalshi.medium.com/rust-caldav-client-from-scratch-da173cfc905d
    // https://sabre.io/dav/building-a-caldav-client/

    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };

    let davclient = Propfind {
        prop: Prop {
            current_user_principal: Some(CurrentUserPrincipal {
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let davclient_xml = yaserde::ser::to_string_with_config(&davclient, &yaserde_cfg).unwrap();

    println!("{}", davclient_xml);

    // https://cloud.selfmade4u.de/remote.php/dav/calendars/Moritz.Hedtke/not-grocy/
    let url = std::env::var("URL").expect("URL required");
    let password = std::env::var("PASSWORD").expect("PASSWORD required");
    let client = reqwest::Client::new();
    let davclient_response_xml = client
        .request(Method::from_bytes(b"PROPFIND").expect("PROPFIND"), &url)
        .header("Depth", 0)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth("Moritz.Hedtke", Some(&password))
        .body(davclient_xml)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", davclient_response_xml);

    let davclient_response: MultiStatus = from_str(davclient_response_xml.as_str())?;

    let davclient_response2: Result<MultiStatus, quick_xml::DeError> =
        quick_xml::de::from_str(&davclient_response_xml);

    println!("{:#?}", davclient_response);
    println!("{:#?}", davclient_response2);

    exit(0);

    let href = davclient_response
        .response
        .first()
        .and_then(|r| {
            r.propstat
                .first()
                .and_then(|p| p.prop.current_user_principal.as_ref())
        })
        .and_then(|u| u.href.as_ref())
        .unwrap();

    println!("href: {}", href);

    let mut parsed_url = Url::parse(&url).unwrap();
    parsed_url.set_path(href);

    let homeset = Propfind {
        the_self: Some(TheSelf {}),
        prop: Prop {
            calendar_home_set: Some(CalendarHomeSet {
                ..Default::default()
            }),
            ..Default::default()
        },
    };

    let homeset_xml = yaserde::ser::to_string_with_config(&homeset, &yaserde_cfg).unwrap();

    println!("{}", homeset_xml);

    let homeset_response_xml = client
        .request(
            Method::from_bytes(b"PROPFIND").expect("PROPFIND"),
            parsed_url.as_str(),
        )
        .header("Depth", 0)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth("Moritz.Hedtke", Some(&password))
        .body(homeset_xml)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", homeset_response_xml);

    let homeset_response: MultiStatus = from_str(homeset_response_xml.as_str())?;

    println!("{:#?}", homeset_response);

    let homeset_href = homeset_response
        .response
        .first()
        .and_then(|r| {
            r.propstat
                .first()
                .and_then(|p| p.prop.calendar_home_set.as_ref())
        })
        .and_then(|u| u.href.as_ref())
        .unwrap();

    let mut parsed_homeset_url = Url::parse(&url).unwrap();
    parsed_homeset_url.set_path(homeset_href);

    let cal = Propfind {
        prop: Prop {
            displayname: Some("".to_string()),
            resourcetype: Some(ResourceType {
                ..Default::default()
            }),
            supported_calendar_component_set: Some(SupportedCalendarComponentSet {
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let cal_xml = yaserde::ser::to_string_with_config(&cal, &yaserde_cfg).unwrap();

    println!("{}", cal_xml);

    let cal_response_xml = client
        .request(
            Method::from_bytes(b"PROPFIND").expect("PROPFIND"),
            parsed_homeset_url.as_str(),
        )
        .header("Depth", 1)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth("Moritz.Hedtke", Some(&password))
        .body(cal_xml)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", cal_response_xml);

    let cal_response: MultiStatus = from_str(cal_response_xml.as_str())?;

    println!("{:#?}", cal_response);

    let mut calendar_href = None;

    'outer: for response in cal_response.response {
        for propstat in response.propstat {
            if let Some("") = propstat.prop.displayname.as_deref() {
                continue 'outer;
            }
            if propstat.prop.displayname != Some("not-grocy".to_string()) {
                continue 'outer;
            }
            println!("{:?}", propstat.prop.displayname);
            if let Some(val) = propstat.prop.supported_calendar_component_set {
                if !val.comp.iter().any(|c| c.name == "VEVENT") {
                    continue 'outer;
                }
            }
        }
        println!("{}", response.href);
        calendar_href = Some(response.href);
    }

    println!("{:?}", calendar_href);

    let mut calendar_url = Url::parse(&url).unwrap();
    calendar_url.set_path(&calendar_href.unwrap());

    // TODO FIXME xml
    let calendar_query = format!(
        r#"
    <c:calendar-query xmlns:d="DAV:" xmlns:c="urn:ietf:params:xml:ns:caldav">
      <d:prop>
        <d:getetag />
        <c:calendar-data />
      </d:prop>
      <c:filter>
        <c:comp-filter name="VCALENDAR">
          <c:comp-filter name="VEVENT" >
            <c:time-range start="{}" end="{}" />
          </c:comp-filter>
        </c:comp-filter>
      </c:filter>
    </c:calendar-query>
"#,
        "20201102T000000Z", "20251107T000000Z"
    );

    let calendar_response_xml = client
        .request(
            Method::from_bytes(b"REPORT").expect("REPORT"),
            calendar_url.as_str(),
        )
        .header("Depth", 1)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth("Moritz.Hedtke", Some(&password))
        .body(calendar_query)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", calendar_response_xml);

    let calendar_response: MultiStatus = from_str(calendar_response_xml.as_str())?;

    println!("{:#?}", calendar_response);

    // https://crates.io/crates/rrule
    // https://crates.io/crates/icalendar (probably good for generation)
    // https://crates.io/crates/ics
    // https://crates.io/crates/ical (probably good for parsing)

    for response in calendar_response.response {
        for propstat in response.propstat {
            let calendar_data = propstat.prop.calendar_data.unwrap();

            let reader = ical::IcalParser::new(calendar_data.as_bytes());

            for line in reader {
                println!("{:#?}", line);
            }
        }
    }

    Ok(())
}
