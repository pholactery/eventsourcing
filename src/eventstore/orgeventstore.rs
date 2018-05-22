extern crate reqwest;
extern crate serde_json;
extern crate uuid;

use super::super::{Event, Result, Error, Kind};
use super::super::cloudevents::CloudEvent;
use super::{EventStore};
use self::reqwest::{Client, StatusCode};
use self::reqwest::header::{Headers, ContentType};
use self::reqwest::mime;

pub struct OrgEventStore {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct StoreEvent {
    event_id: String,
    event_type: String,
    data: serde_json::Value
}

impl OrgEventStore {
    pub fn new(host: &str, port: u16) -> OrgEventStore {
        OrgEventStore {
            host: host.to_owned(), port:port
        }
    }
}

impl EventStore for OrgEventStore {
    fn append(&self, evt: impl Event, stream: &str) -> Result<CloudEvent> {

        let ce: CloudEvent = evt.into();
        let se = vec![StoreEvent {
            event_id: ce.event_id.to_owned(),
            event_type: ce.event_type.to_owned(),
            data: ce.data.clone(),
        }];
        let serialized = serde_json::to_string(&se);
        println!("about to post - {}", serialized.unwrap());
        let client = reqwest::Client::new();
        let url = format!("http://{}:{}/streams/{}", self.host, self.port, stream);
        let mut headers = Headers::new();

        let evtstoretype = "application/vnd.eventstore.events+json".parse::<mime::Mime>().unwrap();
        headers.set(ContentType(evtstoretype));

        match client.post(&url).json(&se).headers(headers)
            .send() {
            Ok(mut response) => {
                if response.status() == StatusCode::Created {
                    Ok(ce)
                } else {
                    Err(Error { kind: Kind::StoreFailure(
                        format!("Failed to post to event store ({})", response.status())
                    )})
                }
            },
            Err(e) => Err(Error { kind: Kind::StoreFailure(
                format!("Failed to post to event store {:?}", e)
            )})
        }

          //      println!("Org event store - {:?}", _se);
        // TODO: submit to Greg Young's event store
        //Ok(ce)
    }
}