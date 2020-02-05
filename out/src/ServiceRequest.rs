use serde::{Deserialize, Serialize};

/// A record of a request for service such as diagnostic investigations, treatments,
/// or operations to be performed.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ServiceRequest {
  /// The status of the order.
  status: String,

  /// The desired performer for doing the requested service.  For example, the
  /// surgeon, dermatopathologist, endoscopist, etc.
  performer: Vec<Reference>,

  /// One or more specimens that the laboratory procedure will use.
  specimen: Vec<Reference>,

  /// Additional clinical information about the patient or specimen that may influence
  /// the services or their interpretations.     This information includes diagnosis,
  /// clinical findings and other observations.  In laboratory ordering these are
  /// typically referred to as "ask at order entry questions (AOEs)".  This includes
  /// observations explicitly requested by the producer (filler) to provide context or
  /// supporting information needed to complete the order. For example,  reporting the
  /// amount of inspired oxygen for blood gas measurements.
  #[serde(rename = "supportingInfo")]
  supporting_info: Vec<Reference>,

  /// Extensions for status
  _status: Element,

  /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
  /// definition that is adhered to in whole or in part by this ServiceRequest.
  #[serde(rename = "instantiatesCanonical")]
  instantiates_canonical: Vec<canonical>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Set this to true if the record is saying that the service/procedure should NOT
  /// be performed.
  #[serde(rename = "doNotPerform")]
  do_not_perform: bool,

  /// Extensions for intent
  _intent: Element,

  /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
  /// determinations that may be needed for delivering the requested service.
  insurance: Vec<Reference>,

  /// An encounter that provides additional information about the healthcare context
  /// in which this request is made.
  encounter: Reference,

  /// Anatomic location where the procedure should be performed. This is the target
  /// site.
  #[serde(rename = "bodySite")]
  body_site: Vec<CodeableConcept>,

  /// The date/time at which the requested service should occur.
  #[serde(rename = "occurrenceTiming")]
  occurrence_timing: Timing,

  /// The request takes the place of the referenced completed or terminated
  /// request(s).
  replaces: Vec<Reference>,

  /// An amount of service being requested which can be a quantity ( for example
  /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
  /// or a range (2.0 to 1.8 Gy per fraction).
  #[serde(rename = "quantityRange")]
  quantity_range: Range,

  /// Extensions for occurrenceDateTime
  #[serde(rename = "_occurrenceDateTime")]
  _occurrence_date_time: Element,

  /// Extensions for language
  _language: Element,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Instructions in terms that are understood by the patient or consumer.
  #[serde(rename = "patientInstruction")]
  patient_instruction: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.

  /// Modifier extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Vec<Extension>,

  /// An explanation or justification for why this service is being requested in coded
  /// or textual form.   This is often for billing purposes.  May relate to the
  /// resources referred to in `supportingInfo`.
  #[serde(rename = "reasonCode")]
  reason_code: Vec<CodeableConcept>,

  /// The date/time at which the requested service should occur.
  #[serde(rename = "occurrenceDateTime")]
  occurrence_date_time: String,

  /// A code that identifies a particular service (i.e., procedure, diagnostic
  /// investigation, or panel of investigations) that have been requested.
  code: CodeableConcept,

  /// Identifiers assigned to this order instance by the orderer and/or the receiver
  /// and/or order fulfiller.
  identifier: Vec<Identifier>,

  /// Extensions for authoredOn
  #[serde(rename = "_authoredOn")]
  _authored_on: Element,

  /// An amount of service being requested which can be a quantity ( for example
  /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
  /// or a range (2.0 to 1.8 Gy per fraction).
  #[serde(rename = "quantityQuantity")]
  quantity_quantity: Quantity,

  /// Indicates how quickly the ServiceRequest should be addressed with respect to
  /// other requests.
  priority: String,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// An amount of service being requested which can be a quantity ( for example
  /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
  /// or a range (2.0 to 1.8 Gy per fraction).
  #[serde(rename = "quantityRatio")]
  quantity_ratio: Ratio,

  /// Any other notes and comments made about the service request. For example,
  /// internal billing notes.
  note: Vec<Annotation>,

  /// A shared identifier common to all service requests that were authorized more or
  /// less simultaneously by a single author, representing the composite or group
  /// identifier.
  requisition: Identifier,

  /// Extensions for asNeededBoolean
  #[serde(rename = "_asNeededBoolean")]
  _as_needed_boolean: Element,

  /// Whether the request is a proposal, plan, an original order or a reflex order.
  intent: String,

  /// Indicates another resource that provides a justification for why this service is
  /// being requested.   May relate to the resources referred to in `supportingInfo`.
  #[serde(rename = "reasonReference")]
  reason_reference: Vec<Reference>,

  /// If a CodeableConcept is present, it indicates the pre-condition for performing
  /// the service.  For example "pain", "on flare-up", etc.
  #[serde(rename = "asNeededBoolean")]
  as_needed_boolean: bool,

  /// The URL pointing to an externally maintained protocol, guideline, orderset or
  /// other definition that is adhered to in whole or in part by this ServiceRequest.
  #[serde(rename = "instantiatesUri")]
  instantiates_uri: Vec<String>,

  /// Key events in the history of the request.
  #[serde(rename = "relevantHistory")]
  relevant_history: Vec<Reference>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// When the request transitioned to being actionable.
  #[serde(rename = "authoredOn")]
  authored_on: dateTime,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// Extensions for doNotPerform
  #[serde(rename = "_doNotPerform")]
  _do_not_perform: Element,

  /// Desired type of performer for doing the requested service.
  #[serde(rename = "performerType")]
  performer_type: CodeableConcept,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// The base language in which the resource is written.
  language: String,

  /// Extensions for instantiatesUri
  #[serde(rename = "_instantiatesUri")]
  _instantiates_uri: Vec<Element>,

  /// Additional details and instructions about the how the services are to be
  /// delivered.   For example, and order for a urinary catheter may have an order
  /// detail for an external or indwelling catheter, or an order for a bandage may
  /// require additional instructions specifying how the bandage should be applied.
  #[serde(rename = "orderDetail")]
  order_detail: Vec<CodeableConcept>,

  /// A code that classifies the service for searching, sorting and display purposes
  /// (e.g. "Surgical Procedure").
  category: Vec<CodeableConcept>,

  /// Extensions for priority
  _priority: Element,

  /// The individual who initiated the request and has responsibility for its
  /// activation.
  requester: Reference,

  /// A reference to the the preferred location(s) where the procedure should actually
  /// happen. E.g. at home or nursing day care center.
  #[serde(rename = "locationReference")]
  location_reference: Vec<Reference>,

  /// Extensions for patientInstruction
  #[serde(rename = "_patientInstruction")]
  _patient_instruction: Element,

  /// Plan/proposal/order fulfilled by this request.
  #[serde(rename = "basedOn")]
  based_on: Vec<Reference>,

  /// The date/time at which the requested service should occur.
  #[serde(rename = "occurrencePeriod")]
  occurrence_period: Period,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// On whom or what the service is to be performed. This is usually a human patient,
  /// but can also be requested on animals, groups of humans or animals, devices such
  /// as dialysis machines, or even locations (typically for environmental scans).
  subject: Reference,

  /// The preferred location(s) where the procedure should actually happen in coded or
  /// free text form. E.g. at home or nursing day care center.
  #[serde(rename = "locationCode")]
  location_code: Vec<CodeableConcept>,

  /// If a CodeableConcept is present, it indicates the pre-condition for performing
  /// the service.  For example "pain", "on flare-up", etc.
  #[serde(rename = "asNeededCodeableConcept")]
  as_needed_codeable_concept: CodeableConcept,

}
