use serde::{Deserialize, Serialize};

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VerificationResult_PrimarySource {
  /// Extensions for validationDate
  #[serde(rename = "_validationDate")]
  _validation_date: Element,

  /// Status of the validation of the target against the primary source (successful;
  /// failed; unknown).
  #[serde(rename = "validationStatus")]
  validation_status: CodeableConcept,

  /// Type of alerts/updates the primary source can send (specific requested changes;
  /// any changes; as defined by source).
  #[serde(rename = "pushTypeAvailable")]
  push_type_available: Vec<CodeableConcept>,

  /// Reference to the primary source.
  who: Reference,

  /// Ability of the primary source to push updates/alerts (yes; no; undetermined).
  #[serde(rename = "canPushUpdates")]
  can_push_updates: CodeableConcept,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

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

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Type of primary source (License Board; Primary Education; Continuing Education;
  /// Postal Service; Relationship owner; Registration Authority; legal source;
  /// issuing source; authoritative source).
  type: Vec<CodeableConcept>,

  /// When the target was validated against the primary source.
  #[serde(rename = "validationDate")]
  validation_date: dateTime,

  /// Method for communicating with the primary source (manual; API; Push).
  #[serde(rename = "communicationMethod")]
  communication_method: Vec<CodeableConcept>,

}
