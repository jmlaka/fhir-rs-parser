use serde::{Deserialize, Serialize};

/// Record of delivery of what is supplied.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SupplyDelivery_SuppliedItem {
  /// The amount of supply that has been dispensed. Includes unit of measure.
  quantity: Quantity,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Identifies the medication, substance or device being dispensed. This is either a
  /// link to a resource representing the details of the item or a code that
  /// identifies the item from a known list.
  #[serde(rename = "itemReference")]
  item_reference: Reference,

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

  /// Identifies the medication, substance or device being dispensed. This is either a
  /// link to a resource representing the details of the item or a code that
  /// identifies the item from a known list.
  #[serde(rename = "itemCodeableConcept")]
  item_codeable_concept: CodeableConcept,

}
