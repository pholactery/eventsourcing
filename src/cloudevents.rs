//! Cloud Events Implementation
//!
//! This module provides an implementation of a [cloud event](https://cloudevents.io/)
//! data structure that conforms to the cloud events v0.1 spec as outlined in
//! [CloudEvents v0.1 Spec](https://github.com/cloudevents/spec/blob/v0.1/spec.md)
//!
//! In the current version of this library, only the _application/json_ content type is supported
//! for the `data` field on the cloud event.
use super::Event;
use chrono::prelude::*;
use serde_json;
use uuid::Uuid;

/// CloudEvent provides a data structure that is JSON-compliant with v0.1 of the CloudEvents
/// specification. This means that any system with which you want to communicate that is
/// also CloudEvents-aware can accept the serialized version of this data structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CloudEvent {
    pub cloud_events_version: String,
    pub event_type: String,
    pub event_type_version: String,
    pub source: String, // URI
    pub event_id: String,
    pub event_time: DateTime<Utc>,
    pub content_type: String,
    pub data: serde_json::Value,
}

impl<E> From<E> for CloudEvent
where
    E: Event,
{
    fn from(source: E) -> Self {
        let raw_data = serde_json::to_string(&source).unwrap();

        CloudEvent {
            cloud_events_version: "0.1".to_owned(),
            event_type: source.event_type().to_owned(),
            event_type_version: source.event_type_version().to_owned(),
            source: source.event_source().to_owned(),
            event_id: Uuid::new_v4().to_hyphenated().to_string(),
            event_time: Utc::now(),
            content_type: "application/json".to_owned(),
            data: serde_json::from_str(&raw_data).unwrap(),
        }
    }
}
