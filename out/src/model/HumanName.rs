#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Period::Period;
use serde_json::value::Value;



/// A human's name with the ability to identify parts and usage.

#[derive(Debug)]
pub struct HumanName<'a> {
  pub value: &'a Value,
}

impl HumanName<'_> {
  /// Identifies the purpose for this name.
  pub fn fhir_use(&self) -> Option<HumanNameUse> {
    if let Some(Value::String(val)) = self.value.get("use") {
      return Some(HumanNameUse::from_string(&val).unwrap());
    }
    return None;
  }

  /// Specifies the entire name as it should be displayed e.g. on an application UI.
  /// This may be provided instead of or as well as the specific parts.
  pub fn text(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("text") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for given
  pub fn _given(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_given") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Indicates the period of time when this name was valid for the named person.
  pub fn period(&self) -> Option<Period> {
    if let Some(val) = self.value.get("period") {
      return Some(Period { value: val });
    }
    return None;
  }

  /// Part of the name that is acquired as a title due to academic, legal, employment
  /// or nobility status, etc. and that appears at the end of the name.
  pub fn suffix(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("suffix") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Part of the name that is acquired as a title due to academic, legal, employment
  /// or nobility status, etc. and that appears at the start of the name.
  pub fn prefix(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("prefix") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// The part of a name that links to the genealogy. In some cultures (e.g. Eritrea)
  /// the family name of a son is the first name of his father.
  pub fn family(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("family") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for suffix
  pub fn _suffix(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_suffix") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for use
  pub fn _use(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_use") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for family
  pub fn _family(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_family") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// Extensions for text
  pub fn _text(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_text") {
      return Some(Element { value: val });
    }
    return None;
  }

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Given name.
  pub fn given(&self) -> Option<Vec<String>> {
    if let Some(Value::Array(val)) = self.value.get("given") {
      return Some(val.into_iter().map(|e| e.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for prefix
  pub fn _prefix(&self) -> Option<Vec<Element>> {
    if let Some(Value::Array(val)) = self.value.get("_prefix") {
      return Some(val.into_iter().map(|e| Element { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum HumanNameUse {
  Usual,
  Official,
  Temp,
  Nickname,
  Anonymous,
  Old,
  Maiden,
}

impl HumanNameUse {
    pub fn from_string(string: &str) -> Option<HumanNameUse> {
      match string {
        "usual" => Some(HumanNameUse::Usual),
        "official" => Some(HumanNameUse::Official),
        "temp" => Some(HumanNameUse::Temp),
        "nickname" => Some(HumanNameUse::Nickname),
        "anonymous" => Some(HumanNameUse::Anonymous),
        "old" => Some(HumanNameUse::Old),
        "maiden" => Some(HumanNameUse::Maiden),
        _ => None,
    }
  }
}

