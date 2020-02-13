#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use serde_json::value::Value;



/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.

#[derive(Debug)]
pub struct ObservationDefinition_QuantitativeDetails<'a> {
  pub value: &'a Value,
}

impl ObservationDefinition_QuantitativeDetails<'_> {
  /// Extensions for decimalPrecision
  pub fn _decimal_precision(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_decimalPrecision") {
      return Some(Element { value: val });
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

  /// Extensions for conversionFactor
  pub fn _conversion_factor(&self) -> Option<Element> {
    if let Some(val) = self.value.get("_conversionFactor") {
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

  /// SI unit used to report quantitative results of observations conforming to this
  /// ObservationDefinition.
  pub fn unit(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("unit") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Customary unit used to report quantitative results of observations conforming to
  /// this ObservationDefinition.
  pub fn customary_unit(&self) -> Option<CodeableConcept> {
    if let Some(val) = self.value.get("customaryUnit") {
      return Some(CodeableConcept { value: val });
    }
    return None;
  }

  /// Factor for converting value expressed with SI unit to value expressed with
  /// customary unit.
  pub fn conversion_factor(&self) -> Option<f64> {
    if let Some(val) = self.value.get("conversionFactor") {
      return Some(val.as_f64().unwrap());
    }
    return None;
  }

  /// Number of digits after decimal separator when the results of such observations
  /// are of type Quantity.
  pub fn decimal_precision(&self) -> Option<i64> {
    if let Some(val) = self.value.get("decimalPrecision") {
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
