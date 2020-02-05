use serde::{Deserialize, Serialize};

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Questionnaire {
  /// A Boolean value to indicate that this questionnaire is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: bool,

  /// The types of subjects that can be the subject of responses created for the
  /// questionnaire.
  #[serde(rename = "subjectType")]
  subject_type: Vec<String>,

  /// The date on which the resource content was last reviewed. Review happens
  /// periodically after approval but does not change the original approval date.
  #[serde(rename = "lastReviewDate")]
  last_review_date: date,

  /// Extensions for name
  _name: Element,

  /// The identifier that is used to identify this version of the questionnaire when
  /// it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the questionnaire author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// Extensions for description
  _description: Element,

  /// Extensions for approvalDate
  #[serde(rename = "_approvalDate")]
  _approval_date: Element,

  /// Explanation of why this questionnaire is needed and why it has been designed as
  /// it has.
  purpose: markdown,

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

  /// Extensions for status
  _status: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate questionnaire
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// A short, descriptive, user-friendly title for the questionnaire.
  title: String,

  /// Extensions for experimental
  _experimental: Element,

  /// The date  (and optionally time) when the questionnaire was published. The date
  /// must change when the business version changes and it must change if the status
  /// code changes. In addition, it should change when the substantive content of the
  /// questionnaire changes.
  date: dateTime,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Extensions for version
  _version: Element,

  /// An absolute URI that is used to identify this questionnaire when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this questionnaire is (or
  /// will be) published. This URL can be the target of a canonical reference. It
  /// SHALL remain the same when the questionnaire is stored on different servers.
  url: String,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A formal identifier that is used to identify this questionnaire when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Vec<Identifier>,

  /// The status of this questionnaire. Enables tracking the life-cycle of the
  /// content.
  status: QuestionnaireStatus,

  /// A free text natural language description of the questionnaire from a consumer's
  /// perspective.
  description: markdown,

  /// A natural language name identifying the questionnaire. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// A legal or geographic region in which the questionnaire is intended to be used.
  jurisdiction: Vec<CodeableConcept>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for subjectType
  #[serde(rename = "_subjectType")]
  _subject_type: Vec<Element>,

  /// A copyright statement relating to the questionnaire and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the questionnaire.
  copyright: markdown,

  /// Extensions for copyright
  _copyright: Element,

  /// Extensions for language
  _language: Element,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// A particular question, question grouping or display text that is part of the
  /// questionnaire.
  item: Vec<Questionnaire_Item>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

  /// The period during which the questionnaire content was or is planned to be in
  /// active use.
  #[serde(rename = "effectivePeriod")]
  effective_period: Period,

  /// Extensions for title
  _title: Element,

  /// Extensions for purpose
  _purpose: Element,

  /// Extensions for url
  _url: Element,

  /// The name of the organization or individual that published the questionnaire.
  publisher: String,

  /// Extensions for publisher
  _publisher: Element,

  /// The base language in which the resource is written.
  language: String,

  /// The date on which the resource content was approved by the publisher. Approval
  /// happens once when the content is officially approved for usage.
  #[serde(rename = "approvalDate")]
  approval_date: date,

  /// Extensions for lastReviewDate
  #[serde(rename = "_lastReviewDate")]
  _last_review_date: Element,

  /// An identifier for this question or group of questions in a particular
  /// terminology such as LOINC.
  code: Vec<Coding>,

  /// The URL of a Questionnaire that this Questionnaire is based on.
  #[serde(rename = "derivedFrom")]
  derived_from: Vec<canonical>,

  /// Extensions for date
  _date: Element,

}

#[derive(Debug, Serialize, Deserialize)]
enum QuestionnaireStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
