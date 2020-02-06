#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::ResourceList::ResourceList;
use crate::model::Identifier::Identifier;
use crate::model::Address::Address;
use crate::model::Meta::Meta;
use crate::model::Extension::Extension;
use crate::model::Practitioner_Qualification::Practitioner_Qualification;
use crate::model::Attachment::Attachment;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Narrative::Narrative;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::HumanName::HumanName;


/// A person who is directly or indirectly involved in the provisioning of
/// healthcare.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Practitioner {
  /// Address(es) of the practitioner that are not role specific (typically home
  /// address).   Work addresses are not typically entered in this property as they
  /// are usually role dependent.
  address: Option<Vec<Address>>,

  /// The metadata about the resource. This is content that is maintained by the
  /// infrastructure. Changes to the content might not always be associated with
  /// version changes to the resource.
  meta: Option<Meta>,

  /// Image of the person.
  photo: Option<Vec<Attachment>>,

  /// Whether this practitioner's record is in active use.
  active: Option<bool>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the resource. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Extensions for implicitRules
  #[serde(rename = "_implicitRules")]
  _implicit_rules: Option<Element>,

  /// A human-readable narrative that contains a summary of the resource and can be
  /// used to represent the content of the resource to a human. The narrative need not
  /// encode all the structured data, but is required to contain sufficient detail to
  /// make it "clinically safe" for a human to just read the narrative. Resource
  /// definitions may define what content should be represented in the narrative to
  /// ensure clinical safety.
  text: Option<Narrative>,

  /// The name(s) associated with the practitioner.
  name: Option<Vec<HumanName>>,

  /// A reference to a set of rules that were followed when the resource was
  /// constructed, and which must be understood when processing the content. Often,
  /// this is a reference to an implementation guide that defines the special rules
  /// along with other profiles etc.
  #[serde(rename = "implicitRules")]
  implicit_rules: Option<String>,

  /// The base language in which the resource is written.
  language: Option<String>,

  /// Extensions for gender
  #[serde(rename = "_gender")]
  _gender: Option<Element>,

  /// A contact detail for the practitioner, e.g. a telephone number or an email
  /// address.
  telecom: Option<Vec<ContactPoint>>,

  /// The date of birth for the practitioner.
  #[serde(rename = "birthDate")]
  birth_date: Option<i32>,

  /// Administrative Gender - the gender that the person is considered to have for
  /// administration and record keeping purposes.
  gender: Option<PractitionerGender>,

  /// An identifier that applies to this person in this role.
  identifier: Option<Vec<Identifier>>,

  /// Extensions for language
  #[serde(rename = "_language")]
  _language: Option<Element>,

  /// The official certifications, training, and licenses that authorize or otherwise
  /// pertain to the provision of care by the practitioner.  For example, a medical
  /// license issued by a medical board authorizing the practitioner to practice
  /// medicine within a certian locality.
  qualification: Option<Vec<Practitioner_Qualification>>,

  /// These resources do not have an independent existence apart from the resource
  /// that contains them - they cannot be identified independently, and nor can they
  /// have their own independent transaction scope.
  contained: Option<Vec<ResourceList>>,

  /// Extensions for birthDate
  #[serde(rename = "_birthDate")]
  _birth_date: Option<Element>,

  /// Extensions for active
  #[serde(rename = "_active")]
  _active: Option<Element>,

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

  /// A language the practitioner can use in patient communication.
  communication: Option<Vec<CodeableConcept>>,

  /// The logical id of the resource, as used in the URL for the resource. Once
  /// assigned, this value never changes.
  id: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum PractitionerGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}
