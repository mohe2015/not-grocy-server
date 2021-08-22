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

    #[yaserde(prefix = "DAV", rename = "filter")]
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
    #[yaserde(prefix = "DAV", rename = "comp-filter")]
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

    #[yaserde(prefix = "DAV", rename = "comp-filter")]
    comp_filter: Vec<CalDAVCompFilter>,
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
