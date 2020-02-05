use serde::{Deserialize, Serialize};

/// A Map of relationships between 2 structures that can be used to transform data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StructureMap {
  /// Extensions for language
  _language: Element,

  /// Explanation of why this structure map is needed and why it has been designed as
  /// it has.
  purpose: markdown,

  /// Extensions for publisher
  _publisher: Element,

  /// A copyright statement relating to the structure map and/or its contents.
  /// Copyright statements are generally legal restrictions on the use and publishing
  /// of the structure map.
  copyright: markdown,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Element,

  /// Other maps used by this map (canonical URLs).
  import: Vec<canonical>,

  /// Extensions for url
  _url: Element,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Vec<ResourceList>,

  /// Extensions for experimental
  _experimental: Element,

  /// Organizes the mapping into manageable chunks for human review/ease of
  /// maintenance.
  group: Vec<StructureMap_Group>,

  /// Extensions for title
  _title: Element,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Vec<Extension>,

  /// The date  (and optionally time) when the structure map was published. The date
  /// must change when the business version changes and it must change if the status
  /// code changes. In addition, it should change when the substantive content of the
  /// structure map changes.
  date: dateTime,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: String,

  /// A Boolean value to indicate that this structure map is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: bool,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Vec<ContactDetail>,

  /// A legal or geographic region in which the structure map is intended to be used.
  jurisdiction: Vec<CodeableConcept>,

  /// Extensions for name
  _name: Element,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Narrative,

  /// The identifier that is used to identify this version of the structure map when
  /// it is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the structure map author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: String,

  /// A free text natural language description of the structure map from a consumer's
  /// perspective.
  description: markdown,

  /// Extensions for description
  _description: Element,

  /// Extensions for date
  _date: Element,

  /// A formal identifier that is used to identify this structure map when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Vec<Identifier>,

  /// Extensions for version
  _version: Element,

  /// A short, descriptive, user-friendly title for the structure map.
  title: String,

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

  /// A structure definition used by this map. The structure definition may describe
  /// instances that are converted, or the instances that are produced.
  structure: Vec<StructureMap_Structure>,

  /// The base language in which the resource is written.
  language: String,

  /// The status of this structure map. Enables tracking the life-cycle of the
  /// content.
  status: StructureMapStatus,

  /// Extensions for purpose
  _purpose: Element,

  /// A natural language name identifying the structure map. This name should be
  /// usable as an identifier for the module by machine processing applications such
  /// as code generation.
  name: String,

  /// The name of the organization or individual that published the structure map.
  publisher: String,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate structure map
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Vec<UsageContext>,

  /// An absolute URI that is used to identify this structure map when it is
  /// referenced in a specification, model, design or an instance; also called its
  /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
  /// address at which at which an authoritative instance of this structure map is (or
  /// will be) published. This URL can be the target of a canonical reference. It
  /// SHALL remain the same when the structure map is stored on different servers.
  url: String,

  /// Extensions for copyright
  _copyright: Element,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: id,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Meta,

}

#[derive(Debug, Serialize, Deserialize)]
enum StructureMapStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
