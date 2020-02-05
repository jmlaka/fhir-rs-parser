use serde::{Deserialize, Serialize};

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contract_Answer {
  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueString")]
  value_string: String,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueReference")]
  value_reference: Reference,

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

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueInteger")]
  value_integer: number,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Response to an offer clause or question text,  which enables selection of values
  /// to be agreed to, e.g., the period of participation, the date of occupancy of a
  /// rental, warrently duration, or whether biospecimen may be used for further
  /// research.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

}
