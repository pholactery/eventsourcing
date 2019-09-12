//! Implementation of Greg Young's Event Store (eventstore.org)
extern crate reqwest;
extern crate serde_json;
extern crate uuid;

use self::reqwest::header::{ContentType, Headers};
use self::reqwest::mime;
use self::reqwest::{Client, StatusCode};
use super::super::cloudevents::CloudEvent;
use super::super::{Error, Event, Kind, Result};
use super::EventStore;

/// Client for the eventstore.org Event Store
pub struct OrgEventStore {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct StoreEvent {
    event_id: String,
    event_type: String,
    data: serde_json::Value,
}

impl OrgEventStore {
    /// Creates a new event store client with the given host name and port number.
    pub fn new(host: &str, port: u16) -> OrgEventStore {
        OrgEventStore {
            host: host.to_owned(),
            port: port,
        }
    }

    fn build_stream_url(&self, stream: &str) -> String {
        format!("http://{}:{}/streams/{}", self.host, self.port, stream)
    }
}

impl Default for OrgEventStore {
    /// Creates an event store client pointing to localhost:2113, the default address
    fn default() -> Self {
        OrgEventStore::new("localhost", 2113)
    }
}

fn generate_headers() -> Headers {
    let mut headers = Headers::new();

    let evtstoretype = "application/vnd.eventstore.events+json"
        .parse::<mime::Mime>()
        .unwrap();
    headers.set(ContentType(evtstoretype));
    headers
}

impl EventStore<CloudEvent, &str> for OrgEventStore {
    fn append(&self, evt: impl Event, store: &str) -> Result<CloudEvent> {
        let ce: CloudEvent = evt.into();
        let se = vec![StoreEvent {
            event_id: ce.event_id.to_owned(),
            event_type: ce.event_type.to_owned(),
            data: ce.data.clone(),
        }];

        let client = reqwest::Client::new();

        let url = self.build_stream_url(stream);
        let headers = generate_headers();

        match client.post(&url).json(&se).headers(headers).send() {
            Ok(mut response) => {
                if response.status() == StatusCode::Created {
                    Ok(ce)
                } else {
                    Err(Error {
                        kind: Kind::StoreFailure(format!(
                            "Failed to post to event store ({})",
                            response.status()
                        )),
                    })
                }
            }
            Err(e) => Err(Error {
                kind: Kind::StoreFailure(format!("Failed to post to event store {:?}", e)),
            }),
        }
    }
}
