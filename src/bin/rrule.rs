use chrono::offset::TimeZone;
use chrono_tz::UTC;
use rrule::{Frequenzy, Options, RRule};

fn main() {
    // https://github.com/fmeringdal/rust-rrule

    // rrule
    let options = Options::new()
        .dtstart(UTC.ymd(2020, 1, 1).and_hms(9, 0, 0))
        .freq(Frequenzy::Daily)
        .build()
        .unwrap();

    let rrule = RRule::new(options);
    let recurrences = rrule.all();
    for recurrence in recurrences {
        println!("{}", recurrence);
    }

    // maybe implement code by myself because it looks hard to read
    // and also can't to building (convert to RRULE string)

    // https://datatracker.ietf.org/doc/html/rfc5545#section-3.3.10
    // https://datatracker.ietf.org/doc/html/rfc5545#section-3.8.5
}
