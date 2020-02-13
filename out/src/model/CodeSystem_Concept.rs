#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeSystem_Designation::CodeSystem_Designation;
use crate::model::Element::Element;
use crate::model::CodeSystem_Property1::CodeSystem_Property1;
use crate::model::Extension::Extension;
use serde_json::value::Value;



/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.

#[derive(Debug)]
pub struct CodeSystem_Concept<'a> {
  pub value: &'a Value,
}

impl CodeSystem_Concept<'_> {
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

  /// The formal definition of the concept. The code system resource does not make
  /// formal definitions required, because of the prevalence of legacy systems.
  /// However, they are highly recommended, as without them there is no formal meaning
  /// associated with the concept.
  pub fn definition(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("definition") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Defines children of a concept to produce a hierarchy of concepts. The nature of
  /// the relationships is variable (is-a/contains/categorizes) - see
  /// hierarchyMeaning.
  pub fn concept(&self) -> Option<Vec<CodeSystem_Concept>> {
    if let Some(Value::Array(val)) = self.value.get("concept") {
      return Some(val.into_iter().map(|e| CodeSystem_Concept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// A code - a text symbol - that uniquely identifies the concept within the code
  /// system.
  pub fn code(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("code") {
      return Some(string.to_string());
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

  /// A human readable string that is the recommended default way to present this
  /// concept to a user.
  pub fn display(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("display") {
      return Some(string.to_string());
    }
    return None;
  }

  /// Extensions for definition
  pub fn _definition(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_definition") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A property value for this concept.
  pub fn property(&self) -> Option<Vec<CodeSystem_Property1>> {
    if let Some(Value::Array(val)) = self.value.get("property") {
      return Some(val.into_iter().map(|e| CodeSystem_Property1 { value: e }).collect::<Vec<_>>());
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

  /// Extensions for code
  pub fn _code(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_code") {
      return Some(Element { value: val });
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

  /// Additional representations for the concept - other languages, aliases,
  /// specialized purposes, used for particular purposes, etc.
  pub fn designation(&self) -> Option<Vec<CodeSystem_Designation>> {
    if let Some(Value::Array(val)) = self.value.get("designation") {
      return Some(val.into_iter().map(|e| CodeSystem_Designation { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

}
