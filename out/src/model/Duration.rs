#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::Element::Element;


/// A length of time.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
  /// Extensions for unit
  #[serde(rename = "_unit")]
  _unit: Option<Element>,

  /// Extensions for code
  #[serde(rename = "_code")]
  _code: Option<Element>,

  /// Extensions for system
  #[serde(rename = "_system")]
  _system: Option<Element>,

  /// Extensions for comparator
  #[serde(rename = "_comparator")]
  _comparator: Option<Element>,

  /// How the value should be understood and represented - whether the actual value is
  /// greater or less than the stated value due to measurement issues; e.g. if the
  /// comparator is "<" , then the real value is < stated value.
  comparator: Option<DurationComparator>,

  /// The identification of the system that provides the coded form of the unit.
  system: Option<String>,

  /// A computer processable form of the unit in some unit representation system.
  code: Option<String>,

  /// Extensions for value
  #[serde(rename = "_value")]
  _value: Option<Element>,

  /// A human-readable form of the unit.
  unit: Option<String>,

  /// The value of the measured amount. The value includes an implicit precision in
  /// the presentation of the value.
  value: Option<f32>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum DurationComparator {
  #[serde(rename = "<")]
  LessThan,

  #[serde(rename = "<=")]
  LessThanOrEqual,

  #[serde(rename = ">=")]
  GreaterThanOrEqual,

  #[serde(rename = ">")]
  GreaterThan,

}
