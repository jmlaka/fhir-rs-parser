#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Reference::Reference;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;


/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim_Diagnosis {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for sequence
  #[serde(rename = "_sequence")]
  _sequence: Option<Element>,

  /// The nature of illness or problem in a coded form or as a reference to an
  /// external defined Condition.
  #[serde(rename = "diagnosisCodeableConcept")]
  diagnosis_codeable_concept: Option<CodeableConcept>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// Indication of whether the diagnosis was present on admission to a facility.
  #[serde(rename = "onAdmission")]
  on_admission: Option<CodeableConcept>,

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
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// The nature of illness or problem in a coded form or as a reference to an
  /// external defined Condition.
  #[serde(rename = "diagnosisReference")]
  diagnosis_reference: Option<Box<Reference>>,

  /// When the condition was observed or the relative ranking.
  #[serde(rename = "type")]
  fhir_type: Option<Vec<CodeableConcept>>,

  /// A package billing code or bundle code used to group products and services to a
  /// particular health condition (such as heart attack) which is based on a
  /// predetermined grouping code system.
  #[serde(rename = "packageCode")]
  package_code: Option<CodeableConcept>,

  /// A number to uniquely identify diagnosis entries.
  sequence: Option<i32>,

}
