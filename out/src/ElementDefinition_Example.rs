use serde::{Deserialize, Serialize};

/// Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ElementDefinition_Example {
  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: ParameterDefinition,

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: DataRequirement,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAddress")]
  value_address: Address,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Element,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Element,

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueContactPoint")]
  value_contact_point: ContactPoint,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAge")]
  value_age: Age,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Element,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAnnotation")]
  value_annotation: Annotation,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDuration")]
  value_duration: Duration,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUrl")]
  value_url: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueId")]
  value_id: String,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: SampledData,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueInteger")]
  value_integer: number,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: ContactDetail,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueContributor")]
  value_contributor: Contributor,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueInstant")]
  value_instant: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueExpression")]
  value_expression: Expression,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: TriggerDefinition,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUsageContext")]
  value_usage_context: UsageContext,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueMeta")]
  value_meta: Meta,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueMarkdown")]
  value_markdown: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueTiming")]
  value_timing: Timing,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueHumanName")]
  value_human_name: HumanName,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: RelatedArtifact,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCount")]
  value_count: Count,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: number,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueString")]
  value_string: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCanonical")]
  value_canonical: String,

  /// Describes the purpose of this example amoung the set of examples.
  label: String,

  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueIdentifier")]
  value_identifier: Identifier,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valuePeriod")]
  value_period: Period,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueReference")]
  value_reference: Reference,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueCode")]
  value_code: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueRatio")]
  value_ratio: Ratio,

  /// Extensions for label
  _label: Element,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueOid")]
  value_oid: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDistance")]
  value_distance: Distance,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: number,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueSignature")]
  value_signature: Signature,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueDosage")]
  value_dosage: Dosage,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueMoney")]
  value_money: Money,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Element,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueRange")]
  value_range: Range,

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

  /// The actual value for the element, which must be one of the types allowed for
  /// this element.
  #[serde(rename = "valueUuid")]
  value_uuid: String,

}
