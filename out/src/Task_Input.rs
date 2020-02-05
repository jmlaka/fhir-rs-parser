use serde::{Deserialize, Serialize};

/// A task to be performed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Task_Input {
  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueString")]
  value_string: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueQuantity")]
  value_quantity: Quantity,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueHumanName")]
  value_human_name: HumanName,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueInstant")]
  value_instant: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueRatio")]
  value_ratio: Ratio,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueRelatedArtifact")]
  value_related_artifact: RelatedArtifact,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDate")]
  value_date: String,

  /// Extensions for valueTime
  #[serde(rename = "_valueTime")]
  _value_time: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueInteger")]
  value_integer: number,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCode")]
  value_code: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueSignature")]
  value_signature: Signature,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueBoolean")]
  value_boolean: bool,

  /// Extensions for valueUnsignedInt
  #[serde(rename = "_valueUnsignedInt")]
  _value_unsigned_int: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCodeableConcept")]
  value_codeable_concept: CodeableConcept,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueSampledData")]
  value_sampled_data: SampledData,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueContactPoint")]
  value_contact_point: ContactPoint,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDataRequirement")]
  value_data_requirement: DataRequirement,

  /// Extensions for valueDateTime
  #[serde(rename = "_valueDateTime")]
  _value_date_time: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAnnotation")]
  value_annotation: Annotation,

  /// Extensions for valueInteger
  #[serde(rename = "_valueInteger")]
  _value_integer: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCoding")]
  value_coding: Coding,

  /// Extensions for valueUuid
  #[serde(rename = "_valueUuid")]
  _value_uuid: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUuid")]
  value_uuid: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueBase64Binary")]
  value_base_6_4_binary: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDistance")]
  value_distance: Distance,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueIdentifier")]
  value_identifier: Identifier,

  /// Extensions for valueDecimal
  #[serde(rename = "_valueDecimal")]
  _value_decimal: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueContributor")]
  value_contributor: Contributor,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAddress")]
  value_address: Address,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueParameterDefinition")]
  value_parameter_definition: ParameterDefinition,

  /// Extensions for valueBoolean
  #[serde(rename = "_valueBoolean")]
  _value_boolean: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUrl")]
  value_url: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueExpression")]
  value_expression: Expression,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueTriggerDefinition")]
  value_trigger_definition: TriggerDefinition,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUsageContext")]
  value_usage_context: UsageContext,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUri")]
  value_uri: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueTiming")]
  value_timing: Timing,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueId")]
  value_id: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAge")]
  value_age: Age,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDosage")]
  value_dosage: Dosage,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valuePositiveInt")]
  value_positive_int: number,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueReference")]
  value_reference: Reference,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueMeta")]
  value_meta: Meta,

  /// Extensions for valueUrl
  #[serde(rename = "_valueUrl")]
  _value_url: Element,

  /// A code or description indicating how the input is intended to be used as part of
  /// the task execution.
  type: CodeableConcept,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueTime")]
  value_time: String,

  /// Extensions for valuePositiveInt
  #[serde(rename = "_valuePositiveInt")]
  _value_positive_int: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueRange")]
  value_range: Range,

  /// Extensions for valueId
  #[serde(rename = "_valueId")]
  _value_id: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valuePeriod")]
  value_period: Period,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueMarkdown")]
  value_markdown: String,

  /// Extensions for valueCanonical
  #[serde(rename = "_valueCanonical")]
  _value_canonical: Element,

  /// Extensions for valueCode
  #[serde(rename = "_valueCode")]
  _value_code: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDecimal")]
  value_decimal: number,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueOid")]
  value_oid: String,

  /// Extensions for valueDate
  #[serde(rename = "_valueDate")]
  _value_date: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCount")]
  value_count: Count,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDateTime")]
  value_date_time: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueContactDetail")]
  value_contact_detail: ContactDetail,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueCanonical")]
  value_canonical: String,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueAttachment")]
  value_attachment: Attachment,

  /// Extensions for valueUri
  #[serde(rename = "_valueUri")]
  _value_uri: Element,

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

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueDuration")]
  value_duration: Duration,

  /// Extensions for valueOid
  #[serde(rename = "_valueOid")]
  _value_oid: Element,

  /// Extensions for valueInstant
  #[serde(rename = "_valueInstant")]
  _value_instant: Element,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueMoney")]
  value_money: Money,

  /// The value of the input parameter as a basic type.
  #[serde(rename = "valueUnsignedInt")]
  value_unsigned_int: number,

  /// Extensions for valueMarkdown
  #[serde(rename = "_valueMarkdown")]
  _value_markdown: Element,

  /// Extensions for valueString
  #[serde(rename = "_valueString")]
  _value_string: Element,

  /// Extensions for valueBase64Binary
  #[serde(rename = "_valueBase64Binary")]
  _value_base_6_4_binary: Element,

}
