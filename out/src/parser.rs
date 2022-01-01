extern crate serde;
extern crate serde_json;

use serde_json::Result;
use std::borrow::Cow;

pub fn fhir_parse(string: &str) -> Option<crate::model::ResourceList::ResourceList> {
    let parsed: Result<serde_json::value::Value> = serde_json::from_str(string);
    match parsed {
        Ok(value) => {
            let resource = crate::model::ResourceList::ResourceList {
                value: Cow::Owned(value),
            };
            return Some(resource);
        }
        Err(_) => {
            return None;
        }
    }
}
