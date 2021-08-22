extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use chrono::{Duration, TimeZone, Utc};
use icalendar::{Calendar, Class, Component, Event, Property, Todo};
use reqwest::{header::CONTENT_TYPE, Method};
use url::Url;
use uuid::Uuid;
use yaserde::de::from_str;

// https://datatracker.ietf.org/doc/html/rfc4918#section-14.16
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "DAV",
    rename = "multistatus",
    default_namespace = "DAV",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct WebDAVMultiStatus {
    #[yaserde(prefix = "DAV", rename = "response")]
    response: Vec<WebDAVResponse>,
}

// https://datatracker.ietf.org/doc/html/rfc4918#section-14.24
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "DAV",
    rename = "response",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct WebDAVResponse {
    #[yaserde(prefix = "DAV", rename = "href")]
    href: String,

    #[yaserde(prefix = "DAV", rename = "propstat")]
    propstat: Vec<WebDAVPropStat>,
}

// https://datatracker.ietf.org/doc/html/rfc4918#section-14.22
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "DAV",
    rename = "propstat"
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct WebDAVPropStat {
    #[yaserde(prefix = "DAV", rename = "prop")]
    prop: WebDAVProp,

    #[yaserde(prefix = "DAV", rename = "status")]
    status: String,
}

// https://datatracker.ietf.org/doc/html/rfc4918#section-14.18
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "DAV",
    rename = "prop",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct WebDAVProp {
    #[yaserde(prefix = "DAV", rename = "resourcetype")]
    resourcetype: Option<ResourceType>,

    #[yaserde(prefix = "DAV", rename = "getetag")]
    getetag: Option<String>,

    #[yaserde(prefix = "CALDAV", rename = "calendar-data")]
    calendar_data: Option<String>,

    #[yaserde(prefix = "CALDAV", rename = "supported-calendar-component-set")]
    supported_calendar_component_set: Option<CalDAVSupportedCalendarComponentSet>,

    #[yaserde(prefix = "CALDAV", rename = "schedule-calendar-transp")]
    schedule_calendar_transp: Option<ScheduleCalendarTransp>,

    #[yaserde(prefix = "oc", rename = "owner-principal")]
    owner_principal: Option<String>,

    #[yaserde(prefix = "DAV", rename = "displayname")]
    displayname: Option<String>,

    #[yaserde(prefix = "CALDAV", rename = "calendar-timezone")]
    calendar_timezone: Option<String>,

    #[yaserde(prefix = "DAV", rename = "current-user-principal")]
    current_user_principal: Option<WebDAVCurrentUserPrincipal>,

    #[yaserde(prefix = "CALDAV", rename = "calendar-home-set")]
    calendar_home_set: Option<CalDAVCalendarHomeSet>,
}

// https://datatracker.ietf.org/doc/html/rfc5397#section-3
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "DAV",
    rename = "current-user-principal",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct WebDAVCurrentUserPrincipal {
    #[yaserde(prefix = "DAV", rename = "href")]
    href: Option<String>,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-6.2.1
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "calendar-home-set",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVCalendarHomeSet {
    #[yaserde(prefix = "DAV", rename = "href")]
    href: Option<String>,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-5.2.3
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "supported-calendar-component-set",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVSupportedCalendarComponentSet {
    #[yaserde(prefix = "CALDAV", rename = "comp")]
    comp: Vec<CalDAVComp>,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.6.1
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "comp",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVComp {
    #[yaserde(attribute)]
    name: String,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct ScheduleCalendarTransp {
    #[yaserde(prefix = "CALDAV", rename = "opaque")]
    opaque: String,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct ResourceType {
    #[yaserde(prefix = "DAV", rename = "collection")]
    collection: Option<Collection>,

    #[yaserde(prefix = "CALDAV", rename = "calendar")]
    calendar: Option<CalDAVCalendar>,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct Collection {}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.1
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVCalendar {}

// https://datatracker.ietf.org/doc/html/rfc4918#section-14.20
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "DAV",
    rename = "propfind",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct WebDAVPropfind {
    #[yaserde(prefix = "DAV", rename = "self")]
    the_self: Option<TheSelf>,

    #[yaserde(prefix = "DAV", rename = "prop")]
    prop: WebDAVProp,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "DAV",
    rename = "propfind",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct TheSelf {}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.5
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "calendar-query",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVCalendarQuery {
    #[yaserde(prefix = "DAV", rename = "prop")]
    prop: WebDAVProp,

    #[yaserde(prefix = "CALDAV", rename = "filter")]
    filter: CalDAVFilter,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "filter",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVFilter {
    #[yaserde(prefix = "CALDAV", rename = "comp-filter")]
    comp_filter: CalDAVCompFilter,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.1
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "comp-filter",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVCompFilter {
    #[yaserde(attribute)]
    name: String,

    #[yaserde(prefix = "CALDAV", rename = "comp-filter")]
    comp_filter: Vec<CalDAVCompFilter>,

    #[yaserde(prefix = "CALDAV", rename = "time-range")]
    time_range: Option<CalDAVTimeRange>,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.9
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "time-range",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVTimeRange {
    #[yaserde(attribute)]
    start: String,

    #[yaserde(attribute)]
    end: String,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.2
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "prop-filter",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVPropFilter {
    #[yaserde(attribute)]
    name: String,

    #[yaserde(prefix = "DAV", rename = "param-filter")]
    comp_filter: Vec<CalDAVParamFilter>,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.3
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "param-filter",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVParamFilter {
    #[yaserde(attribute)]
    name: String,

    #[yaserde(prefix = "DAV", rename = "is-not-defined")]
    is_not_defined: Option<CalDAVIsNotDefined>,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "CALDAV",
    rename = "param-filter",
    namespace = "DAV: DAV:",
    namespace = "CALDAV: urn:ietf:params:xml:ns:caldav"
)]
struct CalDAVIsNotDefined {}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // webdav
    // https://datatracker.ietf.org/doc/html/rfc4918
    // webdav principal https://datatracker.ietf.org/doc/html/rfc5397

    // caldav
    // https://datatracker.ietf.org/doc/html/rfc4791
    // https://datatracker.ietf.org/doc/html/rfc6638

    // DONE https://github.com/marshalshi/caldav-client-rust
    // DONE https://marshalshi.medium.com/rust-caldav-client-from-scratch-da173cfc905d
    // IMPORTANT https://sabre.io/dav/building-a-caldav-client/

    let yaserde_cfg = yaserde::ser::Config {
        perform_indent: true,
        ..Default::default()
    };

    let davclient = WebDAVPropfind {
        prop: WebDAVProp {
            current_user_principal: Some(WebDAVCurrentUserPrincipal {
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let davclient_xml = yaserde::ser::to_string_with_config(&davclient, &yaserde_cfg).unwrap();

    println!("{}", davclient_xml);

    // http://moritz:moritz@10.233.2.2:5232/.web
    // URL=http://10.233.2.2:5232/moritz PASSWORD=moritz cargo run --bin caldavtest
    // https://cloud.selfmade4u.de/remote.php/dav/calendars/Moritz.Hedtke/not-grocy/
    let url = std::env::var("URL").expect("URL required");
    let username = std::env::var("USERNAME").expect("USERNAME required");
    let password = std::env::var("PASSWORD").expect("PASSWORD required");
    let client = reqwest::Client::new();

    let calendar_url = Url::parse(&url).unwrap();

    let calendar_query = CalDAVCalendarQuery {
        prop: WebDAVProp {
            getetag: Some("".to_string()),
            calendar_data: Some("".to_string()),
            ..Default::default()
        },
        filter: CalDAVFilter {
            comp_filter: CalDAVCompFilter {
                name: "VCALENDAR".to_string(),
                comp_filter: vec![CalDAVCompFilter {
                    name: "VEVENT".to_string(),
                    comp_filter: vec![],
                    time_range: Some(CalDAVTimeRange {
                        start: "20201102T000000Z".to_string(),
                        end: "20251107T000000Z".to_string(),
                    }),
                }],
                ..Default::default()
            },
        },
    };

    let calendar_query_xml =
        yaserde::ser::to_string_with_config(&calendar_query, &yaserde_cfg).unwrap();

    println!("{}", calendar_query_xml);

    let calendar_response_xml = client
        .request(
            Method::from_bytes(b"REPORT").expect("REPORT"),
            calendar_url.as_str(),
        )
        .header("Depth", 1)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth(&username, Some(&password))
        .body(calendar_query_xml)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", calendar_response_xml);

    let calendar_response: WebDAVMultiStatus = from_str(calendar_response_xml.as_str())?;

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
                println!("aaa {:#?}", line);
            }
        }
    }

    let event = Event::new()
        .summary("test event")
        .description("here I have something really important to do")
        .starts(Utc::now())
        .class(Class::Confidential)
        .ends(Utc::now() + Duration::days(1))
        .append_property(
            Property::new("TEST", "FOOBAR")
                .add_parameter("IMPORTANCE", "very")
                .add_parameter("DUE", "tomorrow")
                .done(),
        )
        .done();

    let _bday = Event::new()
        .all_day(Utc.ymd(2020, 3, 15))
        .summary("My Birthday")
        .description(
            r#"Hey, I'm gonna have a party
    BYOB: Bring your own beer.
    Hendrik"#,
        )
        .done();

    let uid = Uuid::new_v4();
    let _todo = Todo::new()
        .summary("Buy some milk")
        .uid(&uid.to_string())
        .done();

    let mut calendar = Calendar::new();
    calendar.push(event);

    println!("{}", calendar);

    let mut calendar_put_url = calendar_url.clone();

    calendar_put_url.set_path(&(calendar_put_url.path().to_string() + &uid.to_string() + ".ics"));

    println!("{}", calendar_put_url);

    let calendar_put_xml = client
        .request(Method::PUT, calendar_put_url.as_str())
        .header(CONTENT_TYPE, "text/calendar; charset=utf-8")
        .basic_auth(&username, Some(&password))
        .body(calendar.to_string())
        .send()
        .await?
        .text()
        .await?;

    println!("{}", calendar_put_xml);

    Ok(())
}
