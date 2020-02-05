use serde::{Deserialize, Serialize};

/// Information about a person that is involved in the care for a patient, but who
/// is not the target of healthcare, nor has a formal responsibility in the care
/// process.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RelatedPerson_Communication {
  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for preferred
  _preferred: Element,

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

  /// The ISO-639-1 alpha 2 code in lower case for the language, optionally followed
  /// by a hyphen and the ISO-3166-1 alpha 2 code for the region in upper case; e.g.
  /// "en" for English, or "en-US" for American English versus "en-EN" for England
  /// English.
  language: CodeableConcept,

  /// Indicates whether or not the patient prefers this language (over other languages
  /// he masters up a certain level).
  preferred: bool,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

}
