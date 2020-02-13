#![allow(unused_imports, non_camel_case_types)]

use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use serde_json::value::Value;



/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim_Procedure<'a> {
  pub value: &'a Value,
}

impl Claim_Procedure<'_> {
  /// When the condition was observed or the relative ranking.
  pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
    if let Some(Value::Array(val)) = self.value.get("type") {
      return Some(val.into_iter().map(|e| CodeableConcept { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// The code or reference to a Procedure resource which identifies the clinical
  /// intervention performed.
  pub fn procedure_reference(&self) -> Option<Reference> {
    if let Some(val) = self.value.get("procedureReference") {
      return Some(Reference { value: val });
    }
    return None;
  }

  /// Date and optionally time the procedure was performed.
  pub fn date(&self) -> Option<String> {
    if let Some(Value::String(string)) = self.value.get("date") {
      return Some(string.to_string());
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

  /// Extensions for sequence
  pub fn _sequence(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_sequence") {
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

  /// The code or reference to a Procedure resource which identifies the clinical
  /// intervention performed.
  pub fn procedure_codeable_concept(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("procedureCodeableConcept") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Unique Device Identifiers associated with this line item.
  pub fn udi(&self) -> Option<Vec<Reference>> {
    if let Some(Value::Array(val)) = self.value.get("udi") {
      return Some(val.into_iter().map(|e| Reference { value: e }).collect::<Vec<_>>());
    }
    return None;
  }

  /// Extensions for date
  pub fn _date(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_date") {
      return Some(Element { value: val });
    }
    return None;
  }

  /// A number to uniquely identify procedure entries.
  pub fn sequence(&self) -> Option<i64> {
    if let Some(val) = self.value.get("sequence") {
      return Some(val.as_i64().unwrap());
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
