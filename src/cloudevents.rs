//! Cloud Events Implementation
//! 
//! This module provides an implementation of a cloud event data structure that conforms to 
//! the cloud events v0.1 spec as outlined at [CloudEvents v0.1 Spec](https://github.com/cloudevents/spec/blob/v0.1/spec.md)
//! 
//! In the current version of this library, only the _application/json_ content type is supported for the `data`
//! field on the cloud event.
use super::{Event, Result};
use chrono::prelude::*;
use serde_json;
use uuid::Uuid;
use serde::Serialize;

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

impl<E> From<E> for CloudEvent where E: Event {
    
    fn from(source:E) -> Self {
        let raw_data = serde_json::to_string(&source).unwrap();

        CloudEvent {
            cloud_events_version: "0.1".to_owned(),
            event_type: source.event_type().to_owned(),
            event_type_version: source.event_type_version().to_owned(),
            source: source.event_source().to_owned(),            
            event_id: Uuid::new_v4().hyphenated().to_string(),
            event_time: Utc::now(),
            content_type: "application/json".to_owned(),            
            data: serde_json::from_str(&raw_data).unwrap(),
        }
    }
}
