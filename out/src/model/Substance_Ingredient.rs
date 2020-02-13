#![allow(unused_imports, non_camel_case_types)]

use crate::model::Ratio::Ratio;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// A homogeneous material with a definite composition.

#[derive(Debug)]
pub struct Substance_Ingredient<'a> {
  pub value: &'a Value,
}

impl Substance_Ingredient<'_> {
  /// The amount of the ingredient in the substance - a concentration ratio.
  pub fn quantity(&self) -> Option<Ratio> {
    if let Some(val) = self.value.get("quantity") {
      return Some(Ratio { value: val });
    }
    return None;
  }

  /// Another substance that is a component of this substance.
  pub fn substance_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("substanceCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Another substance that is a component of this substance.
  pub fn substance_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("substanceReference") {
      return Some(Reference { value: val });
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
