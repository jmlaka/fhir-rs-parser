#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ContactDetail::ContactDetail;
use crate::model::Narrative::Narrative;
use crate::model::Identifier::Identifier;
use crate::model::UsageContext::UsageContext;
use crate::model::ConceptMap_Group::ConceptMap_Group;
use crate::model::Extension::Extension;
use crate::model::Element::Element;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Meta::Meta;
use crate::model::ResourceList::ResourceList;


/// A statement of relationships from one set of concepts to one or more other
/// concepts - either concepts in code systems, or data element/data element
/// concepts, or classes in class models.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConceptMap {
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

  /// Extensions for publisher
  #[serde(rename = "_publisher")]
  _publisher: Option<Element>,

  /// A free text natural language description of the concept map from a consumer's
  /// perspective.
  description: Option<String>,

  /// Extensions for version
  #[serde(rename = "_version")]
  _version: Option<Element>,

  /// A legal or geographic region in which the concept map is intended to be used.
  jurisdiction: Option<Vec<CodeableConcept>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// A short, descriptive, user-friendly title for the concept map.
  title: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Identifier for the source value set that contains the concepts that are being
  /// mapped and provides context for the mappings.
  #[serde(rename = "sourceUri")]
  source_uri: Option<String>,

  /// Identifier for the source value set that contains the concepts that are being
  /// mapped and provides context for the mappings.
  #[serde(rename = "sourceCanonical")]
  source_canonical: Option<String>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for date
  #[serde(rename = "_date")]
  _date: Option<Element>,

  /// An absolute URI that is used to identify this concept map when it is referenced
  /// in a specification, model, design or an instance; also called its canonical
  /// identifier. This SHOULD be globally unique and SHOULD be a literal address at
  /// which at which an authoritative instance of this concept map is (or will be)
  /// published. This URL can be the target of a canonical reference. It SHALL remain
  /// the same when the concept map is stored on different servers.
  url: Option<String>,

  /// The date  (and optionally time) when the concept map was published. The date
  /// must change when the business version changes and it must change if the status
  /// code changes. In addition, it should change when the substantive content of the
  /// concept map changes.
  date: Option<String>,

  /// The name of the organization or individual that published the concept map.
  publisher: Option<String>,

  /// Extensions for url
  #[serde(rename = "_url")]
  _url: Option<Element>,

  /// Extensions for sourceCanonical
  #[serde(rename = "_sourceCanonical")]
  _source_canonical: Option<Element>,

  /// A Boolean value to indicate that this concept map is authored for testing
  /// purposes (or education/evaluation/marketing) and is not intended to be used for
  /// genuine usage.
  experimental: Option<bool>,

  /// Extensions for sourceUri
  #[serde(rename = "_sourceUri")]
  _source_uri: Option<Element>,

  /// A natural language name identifying the concept map. This name should be usable
  /// as an identifier for the module by machine processing applications such as code
  /// generation.
  name: Option<String>,

  /// Extensions for targetCanonical
  #[serde(rename = "_targetCanonical")]
  _target_canonical: Option<Element>,

  /// Extensions for title
  #[serde(rename = "_title")]
  _title: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Extensions for status
  #[serde(rename = "_status")]
  _status: Option<Element>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for copyright
  #[serde(rename = "_copyright")]
  _copyright: Option<Element>,

  /// Explanation of why this concept map is needed and why it has been designed as it
  /// has.
  purpose: Option<String>,

  /// Extensions for targetUri
  #[serde(rename = "_targetUri")]
  _target_uri: Option<Element>,

  /// Extensions for purpose
  #[serde(rename = "_purpose")]
  _purpose: Option<Element>,

  /// Extensions for experimental
  #[serde(rename = "_experimental")]
  _experimental: Option<Element>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

  /// Contact details to assist a user in finding and communicating with the
  /// publisher.
  contact: Option<Vec<ContactDetail>>,

  /// A copyright statement relating to the concept map and/or its contents. Copyright
  /// statements are generally legal restrictions on the use and publishing of the
  /// concept map.
  copyright: Option<String>,

  /// The identifier that is used to identify this version of the concept map when it
  /// is referenced in a specification, model, design or instance. This is an
  /// arbitrary value managed by the concept map author and is not expected to be
  /// globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
  /// managed version is not available. There is also no expectation that versions can
  /// be placed in a lexicographical sequence.
  version: Option<String>,

  /// A formal identifier that is used to identify this concept map when it is
  /// represented in other formats, or referenced in a specification, model, design or
  /// an instance.
  identifier: Option<Identifier>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// The status of this concept map. Enables tracking the life-cycle of the content.
  status: Option<ConceptMapStatus>,

  /// The target value set provides context for the mappings. Note that the mapping is
  /// made between concepts, not between value sets, but the value set provides
  /// important context about how the concept mapping choices are made.
  #[serde(rename = "targetCanonical")]
  target_canonical: Option<String>,

  /// The target value set provides context for the mappings. Note that the mapping is
  /// made between concepts, not between value sets, but the value set provides
  /// important context about how the concept mapping choices are made.
  #[serde(rename = "targetUri")]
  target_uri: Option<String>,

  /// The content was developed with a focus and intent of supporting the contexts
  /// that are listed. These contexts may be general categories (gender, age, ...) or
  /// may be references to specific programs (insurance plans, studies, ...) and may
  /// be used to assist with indexing and searching for appropriate concept map
  /// instances.
  #[serde(rename = "useContext")]
  use_context: Option<Vec<UsageContext>>,

  /// Extensions for description
  #[serde(rename = "_description")]
  _description: Option<Element>,

  /// A group of mappings that all have the same source and target system.
  group: Option<Vec<ConceptMap_Group>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConceptMapStatus {
  #[serde(rename = "draft")]
  Draft,

  #[serde(rename = "active")]
  Active,

  #[serde(rename = "retired")]
  Retired,

  #[serde(rename = "unknown")]
  Unknown,

}
