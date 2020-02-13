#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A reference to a code defined by a terminology system.

#[derive(Debug)]
pub struct Coding<'a> {
  pub value: &'a Value,
}

impl Coding<'_> {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  pub fn extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("extension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for version
  pub fn _version(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_version") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Indicates that this coding was chosen by a user directly - e.g. off a pick list
  /// of available items (codes or displays).
  pub fn user_selected(&self) -> Option<bool> {
    if let Some(val) = self.value.get("userSelected") {
      return Some(val.as_bool().unwrap());
    }
    return None;
  }

  /// Extensions for userSelected
  pub fn _user_selected(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_userSelected") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A representation of the meaning of the code in the system, following the rules
  /// of the system.
  pub fn display(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("display") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for code
  pub fn _code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_code") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for system
  pub fn _system(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_system") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// The version of the code system which was used when choosing this code. Note that
  /// a well-maintained code system does not need the version reported, because the
  /// meaning of codes is consistent across versions. However this cannot consistently
  /// be assured, and when the meaning is not guaranteed to be consistent, the version
  /// SHOULD be exchanged.
  pub fn version(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("version") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The identification of the code system that defines the meaning of the symbol in
  /// the code.
  pub fn system(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("system") {
      return Some(string.to_string());
    }
    return None;
  }

  /// A symbol in syntax defined by the system. The symbol may be a predefined code or
  /// an expression in a syntax defined by the coding system (e.g. post-coordination).
  pub fn code(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("code") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for display
  pub fn _display(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_display") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

}
