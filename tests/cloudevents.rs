extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate eventsourcing;
extern crate serde_json;
#[macro_use]
extern crate eventsourcing_derive;
extern crate chrono;

use chrono::prelude::*;
#[cfg(feature = "eventstore")]
use eventsourcing::prelude::*;

const DOMAIN_VERSION: &str = "1.0";

#[derive(Serialize, Deserialize, Event)]
#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://github.com/pholactery/eventsourcing/tests/integration")]
enum TestEvent {
    Sample { val1: u32, val2: u32, val3: String },
}

#[cfg(feature = "eventstore")]
#[test]
fn cloud_event_roundtrip() {
    // ensure that we can produce a cloud event with an arbitrary nested JSON value in the data
    // field and serialize it, then deserialize it, and still be able to convert the serde_json::Value
    // back into the original raw event type.
    let se = TestEvent::Sample {
        val1: 1,
        val2: 2,
        val3: "hello".to_owned(),
    };

    let ce = CloudEvent {
        cloud_events_version: "0.1".to_owned(),
        event_type: "testevent.sample".to_owned(),
        event_type_version: "1.0".to_owned(),
        source: "events://test/source".to_owned(),
        event_id: "abc12345-1111".to_owned(),
        event_time: Utc::now(),
        content_type: "application/json".to_owned(),
        data: serde_json::from_str(&serde_json::to_string(&se).unwrap()).unwrap(),
    };

    let s = serde_json::to_string(&ce).unwrap();
    let round_trip: CloudEvent = serde_json::from_str(&s).unwrap();
    let evtype = round_trip.event_type.clone();
    let evtype_ver = round_trip.event_type_version.clone();
    let event2: TestEvent = round_trip.into();

    assert_eq!(evtype, "testevent.sample");
    assert_eq!(evtype_ver, DOMAIN_VERSION);

    let TestEvent::Sample { val1, val2, val3 } = se;
    let TestEvent::Sample {
        val1: tval1,
        val2: tval2,
        val3: tval3,
    } = event2;
    assert_eq!(val1, tval1);
    assert_eq!(val2, tval2);
    assert_eq!(val3, tval3);
}
