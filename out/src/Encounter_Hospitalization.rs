use serde::{Deserialize, Serialize};

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Encounter_Hospitalization {
  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// Diet preferences reported by the patient.
  #[serde(rename = "dietPreference")]
  diet_preference: Vec<CodeableConcept>,

  /// Special courtesies (VIP, board member).
  #[serde(rename = "specialCourtesy")]
  special_courtesy: Vec<CodeableConcept>,

  /// From where patient was admitted (physician referral, transfer).
  #[serde(rename = "admitSource")]
  admit_source: CodeableConcept,

  /// Location/organization to which the patient is discharged.
  destination: Reference,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Category or kind of location after discharge.
  #[serde(rename = "dischargeDisposition")]
  discharge_disposition: CodeableConcept,

  /// The location/organization from which the patient came before admission.
  origin: Reference,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Any special requests that have been made for this hospitalization encounter,
  /// such as the provision of specific equipment or other things.
  #[serde(rename = "specialArrangement")]
  special_arrangement: Vec<CodeableConcept>,

  /// Pre-admission identifier.
  #[serde(rename = "preAdmissionIdentifier")]
  pre_admission_identifier: Identifier,

  /// Whether this hospitalization is a readmission and why if known.
  #[serde(rename = "reAdmission")]
  re_admission: CodeableConcept,

}
