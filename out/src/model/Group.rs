#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Narrative::Narrative;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use crate::model::Group_Member::Group_Member;
use crate::model::Identifier::Identifier;
use crate::model::Group_Characteristic::Group_Characteristic;
use crate::model::CodeableConcept::CodeableConcept;


/// Represents a defined collection of entities that may be discussed or acted upon
/// collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
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

  /// A unique business identifier for this group.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// Provides a specific type of resource the group includes; e.g. "cow", "syringe",
  /// etc.
  code: Option<CodeableConcept>,

  /// A count of the number of resource instances that are part of the group.
  quantity: Option<u32>,

  /// Extensions for type
  #[serde(rename = "_type")]
  _type: Option<Element>,

  /// Identifies the broad classification of the kind of resources the group includes.
  #[serde(rename = "type")]
  fhir_type: Option<GroupType>,

  /// Indicates whether the record for the group is available for use or is merely
  /// being retained for historical purposes.
  active: Option<bool>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// A label assigned to the group for human identification and communication.
  name: Option<String>,

  /// Extensions for active
  #[serde(rename = "_active")]
  _active: Option<Element>,

  /// Extensions for actual
  #[serde(rename = "_actual")]
  _actual: Option<Element>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for name
  #[serde(rename = "_name")]
  _name: Option<Element>,

  /// Extensions for quantity
  #[serde(rename = "_quantity")]
  _quantity: Option<Element>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Entity responsible for defining and maintaining Group characteristics and/or
  /// registered members.
  #[serde(rename = "managingEntity")]
  managing_entity: Option<Box<Reference>>,

  /// Identifies the resource instances that are members of the group.
  member: Option<Vec<Group_Member>>,

  /// If true, indicates that the resource refers to a specific group of real
  /// individuals.  If false, the group defines a set of intended individuals.
  actual: Option<bool>,

  /// Identifies traits whose presence r absence is shared by members of the group.
  characteristic: Option<Vec<Group_Characteristic>>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum GroupType {
  #[serde(rename = "person")]
  Person,

  #[serde(rename = "animal")]
  Animal,

  #[serde(rename = "practitioner")]
  Practitioner,

  #[serde(rename = "device")]
  Device,

  #[serde(rename = "medication")]
  Medication,

  #[serde(rename = "substance")]
  Substance,

}
