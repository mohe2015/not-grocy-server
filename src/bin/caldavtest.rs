//extern crate serde;

extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use chrono::{Duration, TimeZone, Utc};
use icalendar::{Calendar, Class, Component, Event, Property, Todo};
use reqwest::{header::CONTENT_TYPE, Method};
use url::Url;
use uuid::Uuid;
use yaserde::de::from_str;
//use quick_xml::de::from_str;

// https://datatracker.ietf.org/doc/html/rfc4791#section-1.2
// TODO FIXME use RFC namespace names

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
    response: Vec<Response>,
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
    propstat: Vec<PropStat>,
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

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
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
    comp: Vec<CalendarComponent>,
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
struct CalendarComponent {
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
    collection: Option<Collection>,

    #[yaserde(prefix = "cal", rename = "calendar")]
    calendar: Option<CalDAVCalendar>,
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
struct CalDAVCalendar {}

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
    #[yaserde(prefix = "d", rename = "self")]
    the_self: Option<TheSelf>,

    #[yaserde(prefix = "d", rename = "prop")]
    prop: Prop,
}

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
struct TheSelf {}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.5
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "cal",
    rename = "calendar-query",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CalendarQuery {
    #[yaserde(prefix = "d", rename = "prop")]
    prop: Prop,

    #[yaserde(prefix = "d", rename = "filter")]
    filter: CalDAVFilter,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "cal",
    rename = "filter",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CalDAVFilter {
    #[yaserde(prefix = "d", rename = "comp-filter")]
    comp_filter: CalDAVCompFilter,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.1
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "cal",
    rename = "comp-filter",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CalDAVCompFilter {
    #[yaserde(attribute)]
    name: String,

    #[yaserde(prefix = "d", rename = "comp-filter")]
    comp_filter: Vec<CalDAVCompFilter>,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.2
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "cal",
    rename = "prop-filter",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CalDAVPropFilter {
    #[yaserde(attribute)]
    name: String,

    #[yaserde(prefix = "d", rename = "param-filter")]
    comp_filter: Vec<CalDAVParamFilter>,
}

// https://datatracker.ietf.org/doc/html/rfc4791#section-9.7.3
#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "cal",
    rename = "param-filter",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CalDAVParamFilter {
    #[yaserde(attribute)]
    name: String,

    #[yaserde(prefix = "d", rename = "is-not-defined")]
    is_not_defined: Option<CalDAVIsNotDefined>,
}

#[derive(Default, Debug, YaDeserialize, YaSerialize, PartialEq)]
#[yaserde(
    prefix = "cal",
    rename = "param-filter",
    namespace = "d: DAV:",
    namespace = "s: http://sabredav.org/ns",
    namespace = "cal: urn:ietf:params:xml:ns:caldav",
    namespace = "cs: http://calendarserver.org/ns/",
    namespace = "oc: http://owncloud.org/ns",
    namespace = "nc: http://nextcloud.org/ns"
)]
struct CalDAVIsNotDefined {}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // webdav
    // https://datatracker.ietf.org/doc/html/rfc4918
    // webdav principal https://www.ietf.org/rfc/rfc5397.txt

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

    // http://moritz:moritz@10.233.2.2:5232/.web
    // URL=http://10.233.2.2:5232/moritz PASSWORD=moritz cargo run --bin caldavtest
    // https://cloud.selfmade4u.de/remote.php/dav/calendars/Moritz.Hedtke/not-grocy/
    let url = std::env::var("URL").expect("URL required");
    let username = std::env::var("USERNAME").expect("USERNAME required");
    let password = std::env::var("PASSWORD").expect("PASSWORD required");
    let client = reqwest::Client::new();

    /*
    let davclient_response_xml = client
        .request(Method::from_bytes(b"PROPFIND").expect("PROPFIND"), &url)
        .header("Depth", 0)
        .header(CONTENT_TYPE, "application/xml")
        .basic_auth(&username, Some(&password))
        .body(davclient_xml)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", davclient_response_xml);

    let davclient_response: MultiStatus = from_str(davclient_response_xml.as_str())?;

    println!("{:#?}", davclient_response);

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
        .basic_auth(&username, Some(&password))
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
        .basic_auth(&username, Some(&password))
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
    calendar_url.set_path(&calendar_href.unwrap());*/

    let calendar_url = Url::parse(&url).unwrap();

    // TODO FIXME xml
    // https://datatracker.ietf.org/doc/html/rfc4791#section-7.8
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
        .basic_auth(&username, Some(&password))
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
