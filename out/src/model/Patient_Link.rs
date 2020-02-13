#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::Reference::Reference;
use serde_json::value::Value;



/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.

#[derive(Debug)]
pub struct Patient_Link<'a> {
  pub value: &'a Value,
}

impl Patient_Link<'_> {
  /// The type of link between this patient resource and another patient resource.
  pub fn fhir_type(&self) -> Option<Patient_LinkType> {
    if let Some(Value::String(val)) = self.value.get("type") {
      return Some(Patient_LinkType::from_string(&val).unwrap());
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

  /// The other patient resource that the link refers to.
  pub fn other(&self) -> Reference {
    Reference {
      value: &self.value["other"],
    }
  }

  /// Extensions for type
  pub fn _type(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_type") {
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

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
    if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
      return Some(val.into_iter().map(|e| Extension { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

}

#[derive(Debug)]
pub enum Patient_LinkType {
  ReplacedBy,
  Replaces,
  Refer,
  Seealso,
}

impl Patient_LinkType {
    pub fn from_string(string: &str) -> Option<Patient_LinkType> {
      match string {
        "replaced-by" => Some(Patient_LinkType::ReplacedBy),
        "replaces" => Some(Patient_LinkType::Replaces),
        "refer" => Some(Patient_LinkType::Refer),
        "seealso" => Some(Patient_LinkType::Seealso),
        _ => None,
    }
  }
}

