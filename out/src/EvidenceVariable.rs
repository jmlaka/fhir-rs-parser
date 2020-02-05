use serde::{Deserialize, Serialize};

/// The EvidenceVariable resource describes a "PICO" element that knowledge
/// (evidence, assertion, recommendation) is about.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EvidenceVariable {
  /// An individual or organization primarily responsible for review of some aspect of
  /// the content.
  reviewer: Vec<ContactDetail>,

  /// An individual or organization responsible for officially endorsing the content
  /// for use in some setting.
  endorser: Vec<ContactDetail>,

  /// The short title provides an alternate title for use in informal descriptive
  /// contexts where the full, formal title is not necessary.
  #[serde(rename = "shortTitle")]
  short_title: String,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// An absolute URI that is used to identify this evidence variable when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this evidence variable is
  /// (or will be) published. This URL can be the target of a canonical reference. It
  /// SHALL remain the same when the evidence variable is stored on different servers.
  url: String,

  /// The identifier that is used to identify this version of the evidence variable
  /// when it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the evidence variable author and is not expected to
  /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence. To provide a version consistent with
  /// the Decision Support Service specification, use the format Major.Minor.Revision
  /// (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the
  /// Decision Support Service specification. Note that a version is required for non-
  /// experimental active artifacts.
  version: String,

  /// Extensions for publisher
  _publisher: Element,

  /// A copyright statement relating to the evidence variable and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the evidence variable.
  copyright: markdown,

  /// A free text natural language description of the evidence variable from a
  /// consumer's perspective.
  description: markdown,

  /// Extensions for copyright
  _copyright: Element,

  /// A formal identifier that is used to identify this evidence variable when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Vec<Identifier>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// A short, descriptive, user-friendly title for the evidence variable.
  title: String,

  /// A human-readable string to clarify or explain concepts about the resource.
  note: Vec<Annotation>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// A legal or geographic region in which the evidence variable is intended to be
  /// used.
  jurisdiction: Vec<CodeableConcept>,

  /// The base language in which the resource is written.
  language: String,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: date,

  /// An individiual or organization primarily involved in the creation and
  /// maintenance of the content.
  author: Vec<ContactDetail>,

  /// An individual or organization primarily responsible for internal coherence of
  /// the content.
  editor: Vec<ContactDetail>,

  /// The type of evidence element, a population, an exposure, or an outcome.
  type: EvidenceVariableType,

  /// Extensions for language
  _language: Element,

  /// A natural language name identifying the evidence variable. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// Extensions for subtitle
  _subtitle: Element,

  /// The name of the organization or individual that published the evidence variable.
  publisher: String,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Element,

  /// Related artifacts such as additional documentation, justification, or
  /// bibliographic references.
  #[serde(rename = "relatedArtifact")]
  related_artifact: Vec<RelatedArtifact>,

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

  /// Extensions for shortTitle
  #[serde(rename = "_shortTitle")]
  _short_title: Element,

  /// A characteristic that defines the members of the evidence element. Multiple
  /// characteristics are applied with "and" semantics.
  characteristic: Vec<EvidenceVariable_Characteristic>,

  /// Extensions for version
  _version: Element,

  /// Extensions for date
  _date: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// An explanatory or alternate title for the EvidenceVariable giving additional
  /// information about its content.
  subtitle: String,

  /// Extensions for description
  _description: Element,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate evidence variable
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: date,

  /// Descriptive topics related to the content of the EvidenceVariable. Topics
  /// provide a high-level categorization grouping types of EvidenceVariables that can
  /// be useful for filtering and searching.
  topic: Vec<CodeableConcept>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// Extensions for name
  _name: Element,

  /// Extensions for url
  _url: Element,

  /// Extensions for status
  _status: Element,

  /// Extensions for type
  _type: Element,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Element,

  /// The period during which the evidence variable content was or is planned to be in
  /// active use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// The date  (and optionally time) when the evidence variable was published. The
  /// date must change when the business version changes and it must change if the
  /// status code changes. In addition, it should change when the substantive content
  /// of the evidence variable changes.
  date: dateTime,

  /// Extensions for title
  _title: Element,

  /// The status of this evidence variable. Enables tracking the life-cycle of the
  /// content.
  status: EvidenceVariableStatus,

}

#[derive(Debug, Serialize, Deserialize)]
enum EvidenceVariableType {
  #[serde(rename = "dichotomous")]
  Dichotomous,

  #[serde(rename = "continuous")]
  Continuous,

  #[serde(rename = "descriptive")]
  Descriptive,

}

#[derive(Debug, Serialize, Deserialize)]
enum EvidenceVariableStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
