use serde::{Deserialize, Serialize};

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExplanationOfBenefit_AddItem {
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

  /// Where the product or service was provided.
  #[serde(rename = "locationCodeableConcept")]
  location_codeable_concept: CodeableConcept,

  /// The numbers associated with notes below which apply to the adjudication of this
  /// item.
  #[serde(rename = "noteNumber")]
  note_number: Vec<positiveInt>,

  /// Where the product or service was provided.
  #[serde(rename = "locationReference")]
  location_reference: Reference,

  /// Extensions for servicedDate
  #[serde(rename = "_servicedDate")]
  _serviced_date: Element,

  /// The second-tier service adjudications for payor added services.
  detail: Vec<ExplanationOfBenefit_Detail1>,

  /// Extensions for subDetailSequence
  #[serde(rename = "_subDetailSequence")]
  _sub_detail_sequence: Vec<Element>,

  /// The sequence number of the details within the claim item which this line is
  /// intended to replace.
  #[serde(rename = "detailSequence")]
  detail_sequence: Vec<positiveInt>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Physical service site on the patient (limb, tooth, etc.).
  #[serde(rename = "bodySite")]
  body_site: CodeableConcept,

  /// The sequence number of the sub-details woithin the details within the claim item
  /// which this line is intended to replace.
  #[serde(rename = "subDetailSequence")]
  sub_detail_sequence: Vec<positiveInt>,

  /// Where the product or service was provided.
  #[serde(rename = "locationAddress")]
  location_address: Address,

  /// The number of repetitions of a service or product.
  quantity: Quantity,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedDate")]
  serviced_date: String,

  /// Extensions for factor
  _factor: Element,

  /// The providers who are authorized for the services rendered to the patient.
  provider: Vec<Reference>,

  /// If the item is not a group then this is the fee for the product or service,
  /// otherwise this is the total of the fees for the details of the group.
  #[serde(rename = "unitPrice")]
  unit_price: Money,

  /// The quantity times the unit price for an additional service or product or
  /// charge.
  net: Money,

  /// The adjudication results.
  adjudication: Vec<ExplanationOfBenefit_Adjudication>,

  /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
  #[serde(rename = "subSite")]
  sub_site: Vec<CodeableConcept>,

  /// Extensions for noteNumber
  #[serde(rename = "_noteNumber")]
  _note_number: Vec<Element>,

  /// Extensions for itemSequence
  #[serde(rename = "_itemSequence")]
  _item_sequence: Vec<Element>,

  /// Item typification or modifiers codes to convey additional context for the
  /// product or service.
  modifier: Vec<CodeableConcept>,

  /// When the value is a group code then this item collects a set of related claim
  /// details, otherwise this contains the product, service, drug or other billing
  /// code for the item.
  #[serde(rename = "productOrService")]
  product_or_service: CodeableConcept,

  /// The date or dates when the service or product was supplied, performed or
  /// completed.
  #[serde(rename = "servicedPeriod")]
  serviced_period: Period,

  /// A real number that represents a multiplier used in determining the overall value
  /// of services delivered and/or goods received. The concept of a Factor allows for
  /// a discount or surcharge multiplier to be applied to a monetary amount.
  factor: decimal,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// Extensions for detailSequence
  #[serde(rename = "_detailSequence")]
  _detail_sequence: Vec<Element>,

  /// Claim items which this service line is intended to replace.
  #[serde(rename = "itemSequence")]
  item_sequence: Vec<positiveInt>,

  /// Identifies the program under which this may be recovered.
  #[serde(rename = "programCode")]
  program_code: Vec<CodeableConcept>,

}
