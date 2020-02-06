#![allow(unused_imports, non_camel_case_types)]

use serde::{Deserialize, Serialize};
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use crate::model::Element::Element;
use crate::model::Address::Address;
use crate::model::Reference::Reference;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Period::Period;


/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient_Contact {
  /// Organization on behalf of which the contact is acting or for which the contact
  /// is working.
  organization: Option<Box<Reference>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element and that modifies the understanding of the element in
  /// which it is contained and/or the understanding of the containing element's
  /// descendants. Usually modifier elements provide negation or qualification. To
  /// make the use of extensions safe and manageable, there is a strict set of
  /// governance applied to the definition and use of extensions. Though any
  /// implementer can define an extension, there is a set of requirements that SHALL
  /// be met as part of the definition of the extension. Applications processing a
  /// resource are required to check for modifier extensions.    Modifier extensions
  /// SHALL NOT change the meaning of any elements on Resource or DomainResource
  /// (including cannot change the meaning of modifierExtension itself).
  #[serde(rename = "modifierExtension")]
  modifier_extension: Option<Vec<Box<Extension>>>,

  /// The nature of the relationship between the patient and the contact person.
  relationship: Option<Vec<CodeableConcept>>,

  /// A name associated with the contact person.
  name: Option<HumanName>,

  /// Address for the contact person.
  address: Option<Address>,

  /// Extensions for gender
  #[serde(rename = "_gender")]
  _gender: Option<Element>,

  /// Unique id for the element within a resource (for internal references). This may
  /// be any string value that does not contain spaces.
  id: Option<String>,

  /// A contact detail for the person, e.g. a telephone number or an email address.
  telecom: Option<Vec<ContactPoint>>,

  /// May be used to represent additional information that is not part of the basic
  /// definition of the element. To make the use of extensions safe and manageable,
  /// there is a strict set of governance  applied to the definition and use of
  /// extensions. Though any implementer can define an extension, there is a set of
  /// requirements that SHALL be met as part of the definition of the extension.
  extension: Option<Vec<Box<Extension>>>,

  /// Administrative Gender - the gender that the contact person is considered to have
  /// for administration and record keeping purposes.
  gender: Option<Patient_ContactGender>,

  /// The period during which this contact person or organization is valid to be
  /// contacted relating to this patient.
  period: Option<Period>,

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Patient_ContactGender {
  #[serde(rename = "male")]
  Male,

  #[serde(rename = "female")]
  Female,

  #[serde(rename = "other")]
  Other,

  #[serde(rename = "unknown")]
  Unknown,

}
