//! Implementation of Greg Young's Event Store (eventstore.org)

#[cfg(feature = "orgeventstore")]
use super::super::cloudevents::CloudEvent;
use super::super::{Error, Event, Kind, Result};
#[cfg(feature = "orgeventstore")]
use super::EventStore;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::StatusCode;

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
            port,
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

fn generate_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/vnd.eventstore.events+json".parse().unwrap(),
    );
    headers
}

impl EventStore for OrgEventStore {
    fn append(&self, evt: impl Event, stream: &str) -> Result<CloudEvent> {
        let ce: CloudEvent = evt.into();
        let se = vec![StoreEvent {
            event_id: ce.event_id.to_owned(),
            event_type: ce.event_type.to_owned(),
            data: ce.data.clone(),
        }];

        let client = reqwest::blocking::Client::new();

        let url = self.build_stream_url(stream);
        let headers = generate_headers();

        match client.post(&url).json(&se).headers(headers).send() {
            Ok(response) => {
                if response.status() == StatusCode::CREATED {
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
