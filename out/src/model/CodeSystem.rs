#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::CodeSystem_Concept::CodeSystem_Concept;
use crate::model::UsageContext::UsageContext;
use crate::model::CodeSystem_Property::CodeSystem_Property;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Narrative::Narrative;
use crate::model::ContactDetail::ContactDetail;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::CodeSystem_Filter::CodeSystem_Filter;


/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem {
  /// A copyright statement relating to the code system and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// code system.
  copyright: Option<String>,

  /// The code system defines a compositional (post-coordination) grammar.
  compositional: Option<bool>,

  /// Canonical reference to the value set that contains the entire code system.
  #[serde(rename = "valueSet")]
  value_set: Option<String>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// The date (and optionally time) when the code system resource was created or
  /// revised.
  status: Option<CodeSystemStatus>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource and that modifies the understanding of the element
  /// that contains it and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer is allowed to define an extension, there is a set of requirements
  /// that SHALL be met as part of the definition of the extension. Applications
  /// processing a resource are required to check for modifier extensions.    Modifier
  /// extensions SHALL NOT change the meaning of any elements on Resource or
  /// DomainResource (including cannot change the meaning of modifierExtension
  /// itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// A natural language name identifying the code system. This name should be usable
  /// as an identifier for the module by machine processing applications such as code
  /// generation.
  name: Option<String>,

  /// Explanation of why this code system is needed and why it has been designed as it
  /// has.
  purpose: Option<String>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// A filter that can be used in a value set compose statement when selecting
  /// concepts using a filter.
  filter: Option<Vec<CodeSystem_Filter>>,

  /// The date  (and optionally time) when the code system was published. The date
  /// must change when the business version changes and it must change if the status
  /// code changes. In addition, it should change when the substantive content of the
  /// code system changes.
  date: Option<String>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// A Boolean value to indicate that this code system is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: Option<bool>,

  /// The meaning of the hierarchy of concepts as represented in this resource.
  #[serde(rename = "hierarchyMeaning")]
  hierarchy_meaning: Option<CodeSystemHierarchyMeaning>,

  /// The total number of concepts defined by the code system. Where the code system
  /// has a compositional grammar, the basis of this count is defined by the system
  /// steward.
  count: Option<u32>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// A legal or geographic region in which the code system is intended to be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// Extensions for versionNeeded
  #[serde(rename = "_versionNeeded")]
  _version_needed: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// An absolute URI that is used to identify this code system when it is referenced
  /// in a specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this code system is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the code system is stored on different servers. This is used in
  /// [Coding](datatypes.html#Coding).system.
  url: Option<String>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for caseSensitive
  #[serde(rename = "_caseSensitive")]
  _case_sensitive: Option<Element>,

  /// The canonical URL of the code system that this code system supplement is adding
  /// designations and properties to.
  supplements: Option<String>,

  /// A property defines an additional slot through which additional information can
  /// be provided about a concept.
  property: Option<Vec<CodeSystem_Property>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A short, descriptive, user-friendly title for the code system.
  title: Option<String>,

  /// Extensions for hierarchyMeaning
  #[serde(rename = "_hierarchyMeaning")]
  _hierarchy_meaning: Option<Element>,

  /// This flag is used to signify that the code system does not commit to concept
  /// permanence across versions. If true, a version must be specified when
  /// referencing this code system.
  #[serde(rename = "versionNeeded")]
  version_needed: Option<bool>,

  /// A free text natural language description of the code system from a consumer's
  /// perspective.
  description: Option<String>,

  /// The identifier that is used to identify this version of the code system when it
  /// is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the code system author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence. This is used in
  /// [Coding](datatypes.html#Coding).version.
  version: Option<String>,

  /// A formal identifier that is used to identify this code system when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// Extensions for compositional
  #[serde(rename = "_compositional")]
  _compositional: Option<Element>,

  /// The extent of the content of the code system (the concepts and codes it defines)
  /// are represented in this resource instance.
  content: Option<CodeSystemContent>,

  /// The name of the organization or individual that published the code system.
  publisher: Option<String>,

  /// Extensions for content
  #[serde(rename = "_content")]
  _content: Option<Element>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate code system
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// If code comparison is case sensitive when codes within this system are compared
  /// to each other.
  #[serde(rename = "caseSensitive")]
  case_sensitive: Option<bool>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for count
  #[serde(rename = "_count")]
  _count: Option<Element>,

  /// Concepts that are in the code system. The concept definitions are inherently
  /// hierarchical, but the definitions must be consulted to determine what the
  /// meanings of the hierarchical relationships are.
  concept: Option<Vec<CodeSystem_Concept>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeSystemStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeSystemHierarchyMeaning {
  #[serde(rename = "grouped-by")]
  GroupedBy,

  #[serde(rename = "is-a")]
  IsA,

  #[serde(rename = "part-of")]
  PartOf,

  #[serde(rename = "classified-with")]
  ClassifiedWith,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeSystemContent {
  #[serde(rename = "not-present")]
  NotPresent,

  #[serde(rename = "example")]
  Example,

  #[serde(rename = "fragment")]
  Fragment,

  #[serde(rename = "complete")]
  Complete,

  #[serde(rename = "supplement")]
  Supplement,

}
