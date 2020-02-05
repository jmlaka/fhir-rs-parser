use serde::{Deserialize, Serialize};

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claim_SupportingInfo {
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

  /// The date when or period to which this information refers.
  #[serde(rename = "timingDate")]
  timing_date: String,

  /// Extensions for timingDate
  #[serde(rename = "_timingDate")]
  _timing_date: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for sequence
  _sequence: Element,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueString")]
  value_string: String,

  /// The date when or period to which this information refers.
  #[serde(rename = "timingPeriod")]
  timing_period: Period,

  /// A number to uniquely identify supporting information entries.
  sequence: positiveInt,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueReference")]
  value_reference: Reference,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// The general class of the information supplied: information; exception; accident,
  /// employment; onset, etc.
  category: CodeableConcept,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// System and code pertaining to the specific information regarding special
  /// conditions relating to the setting, treatment or patient  for which care is
  /// sought.
  code: CodeableConcept,

  /// Provides the reason in the situation where a reason code is required in addition
  /// to the content.
  reason: CodeableConcept,

  /// Additional data or information such as resources, documents, images etc.
  /// including references to the data or the actual inclusion of the data.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

}
