#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;



/// The clinical particulars - indications, contraindications etc. of a medicinal
/// product, including for regulatory purposes.

#[derive(Debug)]
pub struct MedicinalProductContraindication_OtherTherapy<'a> {
  pub value: &'a Value,
}

impl MedicinalProductContraindication_OtherTherapy<'_> {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  pub fn id(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("id") {
      return Some(string.to_string());
    }
    return None;
  }

  /// The type of relationship between the medicinal product indication or
  /// contraindication and another therapy.
  pub fn therapy_relationship_type(&self) -> CodeableConcept {
    CodeableConcept {
      value: &self.value["therapyRelationshipType"],
    }
  }

  /// Reference to a specific medication (active substance, medicinal product or class
  /// of products) as part of an indication or contraindication.
  pub fn medication_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("medicationCodeableConcept") {
      return Some(CodeableConcept { value: val });
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

  /// Reference to a specific medication (active substance, medicinal product or class
  /// of products) as part of an indication or contraindication.
  pub fn medication_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("medicationReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

}
